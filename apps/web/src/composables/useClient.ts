import axios from "axios";
import ollama, { type Message, type Tool } from "ollama";
import { defineStore } from "pinia";

const baseURL = import.meta.env.VITE_BASE_URL;

const http = axios.create({
  baseURL,
});

export type Entity = number;

export type Vec2 = [number, number];

export type Action = { Move: Vec2 } | { Attack: Entity };

export type Observe = {
  position: Vec2;
  minions: {
    entity: Entity;
    position: Vec2;
    health: number;
  };
};

const tools: Tool[] = [
  {
    type: "function",
    function: {
      description: "Basic attack the target",
      name: "Attack",
      parameters: {
        type: "object",
        properties: { entity: { type: "number", description: "Target entity ID" } },
        required: ["entity"],
      },
    },
  },
  {
    type: "function",
    function: {
      description: "Move to specified coordinates",
      name: "Move",
      parameters: {
        type: "object",
        properties: {
          position: { type: "array", items: { type: "number" }, description: "Target position, format: [x, y]" },
        },
        required: ["position"],
      },
    },
  },
  {
    type: "function",
    function: {
      description: "Do nothing",
      name: "Nothing",
    },
  },
];

async function getObservation() {
  const res = await http.get("/observe");
  return res.data as Observe;
}

async function getAction(content: string, onNewMessage: (message: Message) => void) {
  const res = await ollama.chat({
    model: "qwen3:8b",
    messages: [
      {
        role: "user",
        content,
      },
    ],
    think: true,
    stream: true,
    tools,
  });

  let action: Action | undefined;

  for await (const chunk of res) {
    onNewMessage(chunk.message);

    const tool_call = chunk.message.tool_calls?.at(0);

    if (tool_call) {
      if (tool_call.function.name == "Attack") {
        const args = tool_call.function.arguments;
        action = {
          Attack: args.entity,
        };
      }
      if (tool_call.function.name == "Move") {
        const args = tool_call.function.arguments;
        action = {
          Move: args.position,
        };
      }
    }
  }

  return action;
}

export const useClientStore = defineStore(
  "client",
  () => {
    const frame = ref(0);
    const thinkFrame = ref(10);
    const playing = ref<boolean>(false);
    const img = ref<string>();
    const observation = ref<Observe>();
    const action = ref<Action>();
    const prompt = ref<string>(
      `You are Fiora, this is the game state you observed.

Game coordinates: [x, y] describes the horizontal position of game objects.

Your attack range is 100

Your passive skill:
- When you deal damage to an enemy from their Vital direction, the enemy takes an additional 5% true damage.
- Vitals are only effective when active, meaning when the active_timer's finish is true, you can proc the Vital.
Notes:
- To determine if you're in a Vital's direction, you cannot just check x for left/right or y for up/down. Reference this code:
pub fn is_in_direction(source: Vec2, target: Vec2, direction: &Direction) -> bool {
    let delta_x = source.x - target.x;
    let delta_y = source.y - target.y;

    let abs_delta_x = delta_x.abs();
    let abs_delta_y = delta_y.abs();

    match direction {
        Direction::Up => delta_y > 0.0 && abs_delta_y > abs_delta_x,

        Direction::Down => delta_y < 0.0 && abs_delta_y > abs_delta_x,

        Direction::Right => delta_x > 0.0 && abs_delta_x > abs_delta_y,

        Direction::Left => delta_x < 0.0 && abs_delta_x > abs_delta_y,
    }
}
- When you are in attack wind-up, it's best not to take action, otherwise the basic attack may be cancelled.
- Try to move to the enemy's Vital direction, and attack only when you're just within attack range.

Please eliminate the target as quickly as possible.`,
    );
    const message = ref<string>();

    async function updateImg() {
      await http.get("/render");
      img.value = `${baseURL}/render?timestamp=${Date.now()}`;
    }

    async function step(think: boolean = true) {
      message.value = "";

      // const afterStep = async () => {
      //   await new Promise((resolve) => setTimeout(resolve, 200));
      //   await updateImg();
      // };

      if (!think) {
        await http.post("/step");
        return;
      }

      try {
        observation.value = await getObservation();
        action.value = await getAction(`${JSON.stringify(observation.value)} ${prompt.value}`, (msg) => {
          if (msg.thinking) {
            message.value += msg.thinking;
          }
          message.value += msg.content;
        });
        if (action.value) {
          await http.post("/step", action.value);
        }
      } catch (error) {
        await http.post("/step");
      }
    }

    async function observe() {
      const res = await http.get("/observe");
      console.log(res.data);
    }

    async function play() {
      playing.value = true;
      await _play();
    }

    async function _play() {
      if (!playing.value) return;
      const think = frame.value % thinkFrame.value == 0;
      console.log(think);

      await step(think);
      frame.value++;
      await _play();
    }

    async function stop() {
      playing.value = false;
    }

    return {
      action,
      img,
      prompt,
      message,
      observation,
      frame,
      thinkFrame,
      playing,
      updateImg,
      step,
      observe,
      play,
      stop,
    };
  },
  {
    persist: {
      pick: ["prompt"],
    },
  },
);
