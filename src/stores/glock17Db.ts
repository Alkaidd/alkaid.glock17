import { ref } from 'vue'
import { defineStore } from 'pinia'
import { dialog } from "@tauri-apps/api"
import { invoke } from '@tauri-apps/api/tauri'
import { message } from 'ant-design-vue';
import type { Dayjs } from 'dayjs'

export const useGlock17DbStore = defineStore('glock17', () => {
  const data = ref<DataItem[]>([])
  const overlapData = ref<OverlapDataItem[]>([])

  const addLine = () => { 
    let tempId = 1;
    if (data.value.length === 0) {
      tempId = 1
    } else {
      tempId = Number(data.value[data.value.length - 1].key ) + 1
    }

    data.value.push({
      key: tempId.toString(),
      id: tempId,
      event: '',
      start_time: '',
      end_time: '',
      person_1: '',
      person_2: '',
      person_3: ''
    })
  }

  const deleteLine = (key: string) => {
    data.value.some((line, index) => {
      if(line.key === key) {
        data.value.splice(index, 1)
      }
    })
    overlapData.value.some((line, index) => {
      if(line.id === Number(key)) {
        overlapData.value.splice(index, 1)
      }
    })
  }

  const checkOverlap = async () => {
    console.log("checkOverlap: ", data.value)
    const res: {data: OverlapDataItem[]} = await invoke('get_overlap_event', { eventData: data.value })
    overlapData.value = res.data
  }

  const isOverlap = (key: string, field: string) => {
    const res = overlapData.value.some(item => item.id === Number(key) &&  item.overlap_fields.some(name => name === field))
    return res
  }

  const importDataFromXlsx = async () => {
    const path = await dialog.open({
      multiple: false,
      directory: false,
      filters: [
        { name: "Excel 工作簿", extensions: ["xlsx"]}
      ]
    })
    
    if (path && typeof path === 'string') {
      const xlsxRes: { data: FormatDataItem[] } = await invoke('parse_data_from_xlsx', { path: path })
      data.value = xlsxRes.data.map(item => {
        return {
          key: String(item.id),
          ...item
        }
      })
    }
  }

  const genXlsxFromData = async () => {
    const path = await dialog.open({
      multiple: false,
      directory: true
    })

    const newPath = (path as string) + '\\export_glock_17.xlsx'
    const res: {status: string} = await invoke('generate_xlsx_from_data', { eventData: data.value, path: newPath})
    if (res.status === "SUCCESS") {
      message.success('导出成功~ ^_^')
    } else {
      message.error('导出失败... T_T')
    }
  }
  return { data, overlapData, addLine, deleteLine, checkOverlap, isOverlap, importDataFromXlsx, genXlsxFromData }
})

export interface FormatDataItem {
  id: number
  event: string
  start_time: string | Dayjs
  end_time: string | Dayjs
  person_1: string
  person_2: string
  person_3?: string
}

export interface DataItem extends FormatDataItem {
  key: string
}

export interface OverlapDataItem extends FormatDataItem {
  overlap_fields: string[]
}