<script setup lang="ts">
import { cloneDeep } from 'lodash-es'
import { reactive, ref } from 'vue'
import type { UnwrapRef } from 'vue'
import { useGlock17DbStore, type DataItem } from '@/stores/glock17Db'
import { storeToRefs } from 'pinia'

const columns = [
  {
    title: '事件',
    dataIndex: 'event',
    width: '20%'
  },
  {
    title: '开始时间',
    dataIndex: 'startTime',
    width: '15%'
  },
  {
    title: '结束时间',
    dataIndex: 'endTime',
    width: '15%'
  },
  {
    title: '人员1',
    dataIndex: 'person1',
    width: '10%'
  },
  {
    title: '人员2',
    dataIndex: 'person2',
    width: '10%'
  },
  {
    title: '人员3',
    dataIndex: 'person3',
    width: '10%'
  },
  {
    title: '操作',
    dataIndex: 'operation'
  }
]

const { data } = storeToRefs(useGlock17DbStore())
const { deleteLine } = useGlock17DbStore()

const dataSource = ref(data)
const editableData: UnwrapRef<Record<string, DataItem>> = reactive({})

const edit = (id: string) => {
  editableData[id] = cloneDeep(dataSource.value.filter((item) => id === item.id)[0])
}
const save = (id: string) => {
  Object.assign(dataSource.value.filter((item) => id === item.id)[0], editableData[id])
  delete editableData[id]
}
const cancel = (id: string) => {
  delete editableData[id]
}
</script>

<template>
  <a-table :columns="columns" :data-source="dataSource" bordered>
    <template #bodyCell="{ column, text, record }">
      <template
        v-if="
          ['event', 'startTime', 'endTime', 'person1', 'person2', 'person3'].includes(
            column.dataIndex
          )
        "
      >
        <div>
          <a-input
            v-if="editableData[record.id]"
            v-model:value="editableData[record.id][column.dataIndex]"
            style="margin: -5px 0"
          />
          <template v-else>
            {{ text }}
          </template>
        </div>
      </template>
      <template v-else-if="column.dataIndex === 'operation'">
        <div>
          <span v-if="editableData[record.id]">
            <a-typography-link @click="save(record.id)" class="mr-2">确认</a-typography-link>
            <a-typography-link @click="cancel(record.id)">取消</a-typography-link>
          </span>
          <span v-else>
            <a class="mr-2" @click="edit(record.id)">编辑</a>
            <a class="mr-2" @click="deleteLine(record.id)">删除</a>
          </span>
        </div>
      </template>
    </template>
  </a-table>
</template>
