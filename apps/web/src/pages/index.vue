<template>
  <div class="bg-background text-foreground data-flow-bg flex h-screen flex-col">
    <!-- 顶部状态栏 - 电竞风格HUD -->
    <div class="border-b border-gray-700 bg-linear-to-r from-gray-900 via-gray-800 to-gray-900 shadow-lg">
      <div class="px-4 sm:px-6 lg:px-8">
        <div class="flex h-16 items-center justify-between">
          <!-- Logo和标题 -->
          <div class="flex items-center gap-4">
            <div
              class="flex h-8 w-8 items-center justify-center rounded-full bg-linear-to-br from-blue-500 to-purple-600 shadow-lg"
            >
              <span class="text-sm font-bold text-white">G</span>
            </div>
            <h1
              class="bg-linear-to-r from-blue-400 to-purple-400 bg-clip-text font-mono text-2xl font-extrabold tracking-tight text-transparent"
            >
              Gloria
            </h1>
          </div>

          <!-- 核心状态指标 -->
          <div class="flex items-center gap-8">
            <div class="group rounded px-3 py-2 text-center transition-all duration-200 hover:bg-gray-800/50">
              <div class="text-xs tracking-wider text-gray-400 uppercase transition-colors group-hover:text-cyan-300">
                Frame
              </div>
              <div
                class="transform font-mono text-lg font-bold text-cyan-400 transition-all duration-200 group-hover:scale-105 group-hover:text-cyan-300"
              >
                {{ clientStore.frame }}
              </div>
            </div>
            <div class="group rounded px-3 py-2 text-center transition-all duration-200 hover:bg-gray-800/50">
              <div class="text-xs tracking-wider text-gray-400 uppercase transition-colors group-hover:text-red-300">
                Enemy HP
              </div>
              <div
                class="transform font-mono text-lg font-bold text-red-400 transition-all duration-200 group-hover:scale-105 group-hover:text-red-300"
              >
                {{ clientStore.observation?.minions.health || "N/A" }}
              </div>
            </div>
            <div class="group rounded px-3 py-2 text-center transition-all duration-200 hover:bg-gray-800/50">
              <div class="text-xs tracking-wider text-gray-400 uppercase transition-colors group-hover:text-green-300">
                Interval
              </div>
              <div
                class="transform font-mono text-lg font-bold text-green-400 transition-all duration-200 group-hover:scale-105 group-hover:text-green-300"
              >
                {{ clientStore.thinkFrame }} 帧
              </div>
            </div>
            <div class="group rounded px-3 py-2 text-center transition-all duration-200 hover:bg-gray-800/50">
              <div class="text-xs tracking-wider text-gray-400 uppercase transition-colors group-hover:text-purple-300">
                Status
              </div>
              <div
                :class="[
                  'rounded px-2 py-1 font-mono text-xs font-bold transition-all duration-300',
                  clientStore.playing
                    ? 'animate-pulse border border-green-500/50 bg-green-500/20 text-green-400'
                    : 'border border-gray-500/50 bg-gray-500/20 text-gray-400',
                ]"
              >
                {{ clientStore.playing ? "ACTIVE" : "IDLE" }}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 主要内容区域 -->
    <div class="flex flex-1 overflow-hidden">
      <!-- 左侧面板 - AI配置和状态 -->
      <div class="flex w-96 flex-col border-l border-gray-800 bg-gray-900">
        <div class="flex flex-1 flex-col overflow-hidden border-gray-800 p-6">
          <h2 class="mb-4 text-lg font-semibold tracking-tight text-blue-400">AI 大脑</h2>

          <Label class="mb-2 block text-sm font-semibold text-gray-400 uppercase">提示词</Label>
          <Textarea
            v-model="clientStore.prompt"
            class="flex-1 border-gray-700 bg-gray-800 font-mono text-sm text-white focus:border-blue-500 focus:ring-blue-500/20"
            rows="5"
            placeholder="Define AI behavior and decision making logic..."
          />
        </div>
        <div class="border-t border-gray-800 bg-gray-900 p-4">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-4">
              <Button
                @click="clientStore.step"
                variant="ghost"
                class="group border border-gray-700 bg-gray-800 text-white transition-all duration-200 hover:border-blue-500/50 hover:bg-gray-700"
                :disabled="clientStore.playing"
              >
                执行一步
              </Button>
              <Button
                @click="clientStore.observe"
                variant="ghost"
                class="group border border-gray-700 bg-gray-800 text-white transition-all duration-200 hover:border-purple-500/50 hover:bg-gray-700"
              >
                观察
              </Button>
            </div>

            <div class="flex items-center gap-3">
              <Button
                @click="clientStore.playing ? clientStore.stop() : clientStore.play()"
                :variant="clientStore.playing ? 'destructive' : 'default'"
                class="group min-w-[80px] shadow-lg transition-all duration-200 hover:shadow-xl"
              >
                {{ clientStore.playing ? "Stop" : "Play" }}
              </Button>
            </div>
          </div>
        </div>
      </div>

      <div class="flex w-96 flex-col border-l border-gray-800 bg-gray-900">
        <!-- AI配置区域 -->

        <!-- AI决策监控 -->
        <div class="flex-1 overflow-y-auto p-6">
          <h3 class="mb-4 text-lg font-semibold tracking-tight text-purple-400">AI Decision Monitor</h3>

          <!-- 当前决策 -->
          <div
            v-if="clientStore.action"
            class="group mb-6 rounded-lg border border-green-500/30 bg-linear-to-r from-green-900/30 via-gray-800 to-gray-900 p-4 shadow-lg transition-all duration-300 hover:shadow-green-500/20"
          >
            <div class="mb-3 flex items-center gap-2 text-xs tracking-wider text-green-400 uppercase">
              <span class="h-1.5 w-1.5 animate-pulse rounded-full bg-green-400"></span>
              Current Action
            </div>
            <div class="font-mono text-sm text-green-300 transition-colors group-hover:text-green-200">
              {{ JSON.stringify(clientStore.action, null, 2) }}
            </div>
          </div>

          <!-- 观察结果 -->
          <div
            v-if="clientStore.observation"
            class="group mb-6 rounded-lg border border-cyan-500/30 bg-linear-to-r from-cyan-900/30 via-gray-800 to-gray-900 p-4 shadow-lg transition-all duration-300 hover:shadow-cyan-500/20"
          >
            <div class="mb-3 flex items-center gap-2 text-xs tracking-wider text-cyan-400 uppercase">
              <span class="h-1.5 w-1.5 animate-pulse rounded-full bg-cyan-400"></span>
              Observation
            </div>
            <div class="font-mono text-sm text-cyan-300 transition-colors group-hover:text-cyan-200">
              {{ JSON.stringify(clientStore.observation, null, 2) }}
            </div>
          </div>

          <!-- 思考过程 -->
          <div
            v-if="clientStore.message"
            class="group mb-6 rounded-lg border border-yellow-500/30 bg-linear-to-r from-yellow-900/30 via-gray-800 to-gray-900 p-4 shadow-lg transition-all duration-300 hover:shadow-yellow-500/20"
          >
            <div class="mb-3 flex items-center gap-2 text-xs tracking-wider text-yellow-400 uppercase">
              <span class="h-1.5 w-1.5 animate-pulse rounded-full bg-yellow-400"></span>
              Thought Process
            </div>
            <div
              class="max-h-32 overflow-y-auto font-mono text-sm text-yellow-300 transition-colors group-hover:text-yellow-200"
            >
              {{ clientStore.message }}
            </div>
          </div>

          <!-- 决策历史 -->
          <div class="space-y-3">
            <div class="mb-3 flex items-center gap-2 text-sm tracking-wider text-gray-400 uppercase">
              <span class="h-1 w-1 rounded-full bg-gray-400"></span>
              Decision History
            </div>
            <div class="space-y-2">
              <div
                class="group rounded border border-gray-700 bg-linear-to-r from-gray-800/50 to-gray-900/50 p-3 transition-all duration-200 hover:border-gray-600"
              >
                <div class="mb-2 flex items-center justify-between">
                  <div class="text-xs text-gray-500">Frame {{ clientStore.frame }}</div>
                  <div class="h-1.5 w-1.5 animate-pulse rounded-full bg-blue-400"></div>
                </div>
                <div class="font-mono text-sm text-gray-300 transition-colors group-hover:text-white">
                  {{ clientStore.action ? "Action executed" : "No action" }}
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- 底部状态栏 -->
        <div class="border-t border-gray-800 bg-gray-900 p-4">
          <div class="flex items-center justify-between text-xs text-gray-400">
            <span>AI Status: {{ clientStore.playing ? "Running" : "Stopped" }}</span>
            <span class="font-mono">Frame: {{ clientStore.frame }}</span>
          </div>
        </div>
      </div>

      <!-- 右侧游戏视图 - 主要焦点区域 -->
      <div class="flex flex-1 flex-col bg-gray-900/50">
        <!-- 游戏画面 -->
        <div class="relative flex-1 border-r border-gray-800">
          <div class="absolute inset-0 bg-linear-to-br from-gray-900 to-black">
            <img v-if="clientStore.img" :src="clientStore.img" alt="游戏画面" class="h-full w-full object-cover" />
            <div v-else class="flex h-full items-center justify-center">
              <div class="text-center text-gray-500">
                <div class="mb-4 text-6xl opacity-50">🎮</div>
                <p class="text-lg">等待游戏数据...</p>
                <p class="mt-2 text-sm text-gray-600">点击更新图像开始</p>
              </div>
            </div>
          </div>

          <!-- 游戏画面覆盖层HUD -->
          <div class="absolute top-4 right-4 left-4 flex items-start justify-between">
            <div class="rounded-lg border border-gray-700 bg-black/80 px-4 py-3 shadow-xl backdrop-blur-sm">
              <div class="text-xs tracking-wider text-gray-400 uppercase">Game View</div>
              <div class="flex items-center gap-2 font-mono text-sm text-cyan-400">
                <span class="h-2 w-2 animate-pulse rounded-full bg-cyan-400"></span>
                Real-time
              </div>
            </div>
            <Button
              @click="clientStore.updateImg"
              variant="ghost"
              class="border border-gray-700 bg-black/80 shadow-lg backdrop-blur-sm transition-all duration-200 hover:border-cyan-500/50 hover:bg-gray-800/80"
              size="sm"
            >
              <span class="mr-2">📸</span>
              Refresh
            </Button>
          </div>
        </div>

        <!-- 底部控制面板 -->
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { useClientStore } from "@/composables/useClient";

const clientStore = useClientStore();
</script>

<style scoped>
::-webkit-scrollbar {
  width: 4px;
}

::-webkit-scrollbar-track {
  background: rgba(55, 65, 81, 0.3);
  border-radius: 2px;
}

::-webkit-scrollbar-thumb {
  background: rgba(156, 163, 175, 0.5);
  border-radius: 2px;
}

::-webkit-scrollbar-thumb:hover {
  background: rgba(156, 163, 175, 0.7);
}

@keyframes glow {
  0%,
  100% {
    box-shadow: 0 0 2px rgba(59, 130, 246, 0.5);
  }
  50% {
    box-shadow: 0 0 8px rgba(59, 130, 246, 0.8);
  }
}

.glow-border {
  animation: glow 2s ease-in-out infinite;
}

@keyframes dataFlow {
  0% {
    background-position: 0% 50%;
  }
  100% {
    background-position: 100% 50%;
  }
}

.data-flow-bg {
  background: linear-gradient(
    90deg,
    rgba(6, 182, 212, 0.1) 0%,
    rgba(139, 92, 246, 0.1) 25%,
    rgba(34, 197, 94, 0.1) 50%,
    rgba(139, 92, 246, 0.1) 75%,
    rgba(6, 182, 212, 0.1) 100%
  );
  background-size: 200% 100%;
  animation: dataFlow 8s linear infinite;
}
</style>
