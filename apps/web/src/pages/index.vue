<template>
  <div class="flex h-screen">
    <img class="h-auto w-1/2 object-cover" :src="clientStore.img" />
    <div class="flex w-1/2 flex-col gap-2 overflow-hidden border-l">
      <div class="flex items-center justify-between p-4">
        <div class="">当前帧：{{ clientStore.frame }}</div>
        <div class="">敌人血量: {{ clientStore.observation?.minions.health }}</div>
        <div class="text-lg font-bold">循环执行间隔：{{ clientStore.thinkFrame }} 帧</div>
      </div>
      <div class="grid w-full gap-2 p-6">
        <Label for="prompt">提示词</Label>
        <Textarea id="prompt" placeholder="请输入提示词" class="h-96" v-model="clientStore.prompt" />
        <p class="text-muted-foreground text-sm">提示词将被发送到模型。</p>
        <Button @click="clientStore.updateImg">更新画面</Button>
        <Button @click="clientStore.step">执行一步</Button>
        <Button @click="clientStore.observe">观察</Button>
        <Button @click="clientStore.play">循环执行</Button>
        <Button @click="clientStore.stop">停止循环</Button>
      </div>
      <div class="flex flex-1 flex-col gap-2 overflow-y-auto p-4">
        <div class="ml-4 text-sm font-bold">
          动作：
          {{ clientStore.action }}
        </div>
        <XMarkdown v-if="clientStore.message" :markdown="clientStore.message" class="vp-raw flex-1 overflow-y-auto" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { XMarkdown } from "vue-element-plus-x";
import { useClientStore } from "@/composables/useClient";

const clientStore = useClientStore();
</script>
