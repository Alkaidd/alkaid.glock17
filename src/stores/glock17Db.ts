import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

export const useGlock17DbStore = defineStore('glock17', () => {
  const data = ref<DataItem[]>([])

  const addLine = () => { 
    let tempId = 1;
    if (data.value.length === 0) {
      tempId = 1
    } else {
      tempId = Number(data.value[data.value.length - 1].id ) + 1
    }
    const id = tempId.toString()
    data.value.push({
      id,
      event: '',
      startTime: '',
      endTime: '',
      person1: '',
      person2: '',
      person3: ''
    })
  }

  const deleteLine = (id: string) => {
    data.value.some((line, index) => {
      if(line.id === id) {
        data.value.splice(index, 1)
      }
    })
  }
  return { data, addLine, deleteLine }
})

export interface DataItem {
  id: string
  event: string
  startTime: string
  endTime: string
  person1: string
  person2: string
  person3?: string
}