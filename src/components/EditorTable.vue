<script setup lang="ts">
import { cloneDeep } from 'lodash-es'
import { reactive, ref, computed, watchEffect } from 'vue'
import type { UnwrapRef } from 'vue'
import { useGlock17DbStore, type DataItem } from '@/stores/glock17Db'
import { storeToRefs } from 'pinia'
import dayjs from 'dayjs'

const columns = [
  {
    title: '事件',
    dataIndex: 'event',
    width: '20%'
  },
  {
    title: '开始时间',
    dataIndex: 'start_time',
    width: '20%'
  },
  {
    title: '结束时间',
    dataIndex: 'end_time',
    width: '20%'
  },
  {
    title: '人员1',
    dataIndex: 'person_1',
    width: '10%'
  },
  {
    title: '人员2',
    dataIndex: 'person_2',
    width: '10%'
  },
  {
    title: '人员3',
    dataIndex: 'person_3',
    width: '10%'
  },
  {
    title: '操作',
    dataIndex: 'operation'
  }
]

const { data, overlapData } = storeToRefs(useGlock17DbStore())
const { deleteLine, isOverlap } = useGlock17DbStore()

const dataSource = ref(data)
const editableData: UnwrapRef<Record<string, DataItem>> = reactive({})

const edit = (key: string) => {
  const tempData = cloneDeep(dataSource.value.filter((item) => key === item.key)[0])
  tempData.start_time = tempData.start_time
    ? dayjs(tempData.start_time, 'YYYY-MM-DD HH:mm:ss')
    : tempData.start_time
  tempData.end_time = tempData.end_time
    ? dayjs(tempData.end_time, 'YYYY-MM-DD HH:mm:ss')
    : tempData.end_time
  editableData[key] = tempData
}
const save = (key: string) => {
  editableData[key].start_time = editableData[key].start_time
    ? dayjs(editableData[key].start_time).format('YYYY-MM-DD HH:mm:ss')
    : editableData[key].start_time
  editableData[key].end_time = editableData[key].end_time
    ? dayjs(editableData[key].end_time).format('YYYY-MM-DD HH:mm:ss')
    : editableData[key].end_time

  Object.assign(dataSource.value.filter((item) => key === item.key)[0], editableData[key])
  delete editableData[key]
}
const cancel = (key: string) => {
  delete editableData[key]
}

const props = defineProps<{
  selectedRowKeys: string[]
  showOverlapOnly: boolean
}>()
watchEffect(() => {
  if (props.showOverlapOnly) {
    dataSource.value = overlapData.value.map((item) => {
      const newItem = {
        key: Number(item.id),
        ...item
      }
      return newItem as unknown as DataItem
    })
  } else {
    dataSource.value = data.value
  }
})

const emit = defineEmits<{
  (e: 'update:selectedRowKeys', keys: string[]): void
}>()
const tableSelectedRowKeys = computed({
  get() {
    return props.selectedRowKeys
  },
  set(newVal) {
    emit('update:selectedRowKeys', newVal)
  }
})

const onSelectChange = (selectedRowKeys: string[]) => {
  tableSelectedRowKeys.value = selectedRowKeys
}
</script>

<template>
  <a-table
    :columns="columns"
    :row-selection="{ selectedRowKeys: tableSelectedRowKeys, onChange: onSelectChange }"
    :data-source="dataSource"
    bordered
  >
    <template #bodyCell="{ column, text, record }">
      <template
        v-if="
          ['event', 'start_time', 'end_time', 'person_1', 'person_2', 'person_3'].includes(
            column.dataIndex
          )
        "
      >
        <div>
          <template v-if="editableData[record.key]">
            <a-date-picker
              v-if="['start_time', 'end_time'].includes(column.dataIndex)"
              v-model:value="editableData[record.key][column.dataIndex as keyof DataItem]"
              format="YYYY-MM-DD HH:mm:ss"
              show-time
              placeholder="选择时间"
            ></a-date-picker>
            <a-textarea
              v-else-if="column.dataIndex === 'event'"
              v-model:value="editableData[record.key][column.dataIndex as keyof DataItem]"
            />
            <a-input
              v-else
              v-model:value="editableData[record.key][column.dataIndex as keyof DataItem]"
            />
          </template>
          <template v-else>
            <span
              :class="{
                'text-red-500': isOverlap(record.key, column.dataIndex),
                'font-bold': isOverlap(record.key, column.dataIndex)
              }"
              class="whitespace-normal break-all"
              >{{ text }}
            </span>
          </template>
        </div>
      </template>
      <template v-else-if="column.dataIndex === 'operation'">
        <div>
          <span v-if="editableData[record.key]">
            <a-typography-link @click="save(record.key)" class="mr-2">确认</a-typography-link>
            <a-typography-link @click="cancel(record.key)">取消</a-typography-link>
          </span>
          <span v-else>
            <a class="mr-2" @click="edit(record.key)">编辑</a>
            <a class="mr-2" @click="deleteLine(record.key)">删除</a>
          </span>
        </div>
      </template>
    </template>
  </a-table>
</template>
