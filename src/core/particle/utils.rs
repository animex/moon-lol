use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use league_core::{ValueFloat, ValueVector3};

pub fn create_black_pixel_texture() -> Image {
    let image = Image::new_fill(
        Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8Unorm,
        default(),
    );

    image
}

#[derive(Debug, Clone)]
pub enum Sampler<T> {
    Constant(T),
    Curve(UnevenSampleAutoCurve<T>),
}

macro_rules! impl_sampler_traits {
    ($Sampler:ty, $Value:ty, $Data:ty, $domain_logic:expr) => {
        impl Curve<$Data> for $Sampler {
            fn sample_clamped(&self, t: f32) -> $Data {
                match self {
                    Self::Constant(v) => *v,
                    Self::Curve(c) => c.sample_clamped(t),
                }
            }

            fn sample_unchecked(&self, t: f32) -> $Data {
                match self {
                    Self::Constant(v) => *v,
                    Self::Curve(c) => c.sample_unchecked(t),
                }
            }

            fn domain(&self) -> Interval {
                $domain_logic
            }
        }

        // --- 我们将重点修改这里的 From Trait 实现 ---
        impl From<$Value> for $Sampler {
            fn from(value: $Value) -> Self {
                // 首先检查是否存在 dynamics 动态数据块
                if let Some(dynamics) = value.dynamics {
                    // 根据样本点的数量来决定如何处理
                    match dynamics.times.len() {
                        0 => {
                            // 如果有 dynamics 块但里面没有点，
                            // 逻辑上应该回退到使用外层的 constant_value
                            Self::Constant(value.constant_value.unwrap_or_default())
                        }
                        1 => {
                            // 核心逻辑：如果只有一个点，它代表一个恒定值。
                            // 这个值就是这个点的值。
                            // 我们可以安全地 unwrap，因为我们已经检查了长度是 1。
                            Self::Constant(dynamics.values.into_iter().next().unwrap())
                        }
                        _ => {
                            // 2 个或更多点
                            // 这是原来的逻辑，尝试创建一条曲线
                            let samples = dynamics.times.into_iter().zip(dynamics.values);
                            match UnevenSampleAutoCurve::new(samples) {
                                Ok(curve) => Self::Curve(curve),
                                Err(_) => {
                                    // 如果因为某些原因（比如时间点不递增）创建曲线失败，
                                    // 安全地回退到 constant_value
                                    Self::Constant(value.constant_value.unwrap_or_default())
                                }
                            }
                        }
                    }
                } else {
                    // 如果根本没有 dynamics 数据块，那么它就是一个简单的常量
                    Self::Constant(value.constant_value.unwrap_or_default())
                }
            }
        }
    };
}

impl_sampler_traits!(Sampler<f32>, ValueFloat, f32, Interval::EVERYWHERE);

impl_sampler_traits!(Sampler<Vec3>, ValueVector3, Vec3, Interval::EVERYWHERE);
