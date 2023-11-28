<script setup lang="ts">
import { ref } from 'vue'
import { useGlock17DbStore } from '@/stores/glock17Db'

import EditorTable from '@/components/EditorTable.vue'

const { addLine } = useGlock17DbStore()
</script>

<template>
  <div class="p-2 flex flex-col w-full h-full">
    <div class="w-full flex justify-between my-2">
      <div>
        <a-button @click="addLine">添加行</a-button>
      </div>
      <div>
        <a-button class="mr-3" type="primary">导入</a-button>
        <a-button>导出</a-button>
      </div>
    </div>
    <div class="w-full flex-1">
      <EditorTable />
    </div>
  </div>
</template>
