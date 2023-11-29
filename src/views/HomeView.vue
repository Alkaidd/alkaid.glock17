<script setup lang="ts">
import { computed, ref } from 'vue'
import { useGlock17DbStore } from '@/stores/glock17Db'

import EditorTable from '@/components/EditorTable.vue'
import { storeToRefs } from 'pinia'

const { addLine, deleteLine, checkOverlap, importDataFromXlsx, genXlsxFromData } =
  useGlock17DbStore()
const { overlapData } = storeToRefs(useGlock17DbStore())

const selectedRowKeys = ref<string[]>([])
const deleteLines = () => {
  selectedRowKeys.value.forEach((key) => {
    deleteLine(key)
  })
  selectedRowKeys.value = []
}

const isShowOverlapOnly = ref(false)
const showOverlapBtnText = computed(() => {
  if (isShowOverlapOnly.value) {
    return '取消仅展示冲突数据'
  }
  return '仅展示冲突数据'
})
const showOverlapOnly = () => {
  isShowOverlapOnly.value = !isShowOverlapOnly.value
}
</script>

<template>
  <div class="p-2 flex flex-col w-full h-full">
    <div class="w-full flex justify-between my-2">
      <div>
        <a-button class="mr-3" @click="addLine" :disabled="isShowOverlapOnly">添加行</a-button>
        <a-button @click="deleteLines">批量删除</a-button>
      </div>
      <div>
        <a-button
          class="mr-3"
          @click="showOverlapOnly"
          :disabled="overlapData.length === 0 && !isShowOverlapOnly"
          >{{ showOverlapBtnText }}</a-button
        >
        <a-button class="mr-3" @click="checkOverlap">检查冲突</a-button>
        <a-button class="mr-3" type="primary" @click="importDataFromXlsx">导入</a-button>
        <a-button @click="genXlsxFromData">导出</a-button>
      </div>
    </div>
    <div class="w-full flex-1">
      <EditorTable v-model:selectedRowKeys="selectedRowKeys" :showOverlapOnly="isShowOverlapOnly" />
    </div>
  </div>
</template>
