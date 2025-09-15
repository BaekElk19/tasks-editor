<template>
  <el-container style="height:100vh">
    <!-- 顶部工具栏 -->
    <el-header>
      <Toolbar
        :taskTypes="taskTypes"
        @import="onImport"
        @export="onExport"
        @validate="onValidate"
        @addRootTask="onAddRootTask"
      />
    </el-header>

    <el-container>
      <!-- 左侧任务树 -->
      <el-aside width="300px">
        <TaskTree
          :tasks="tasks"
          :taskTypes="taskTypes"
          v-model:selected="selectedTask"
        />
      </el-aside>

      <!-- 右侧表单 -->
      <el-main>
        <TaskForm :task="selectedTask" :taskTypes="taskTypes" />
      </el-main>
    </el-container>
  </el-container>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import Toolbar from './components/Toolbar.vue'
import TaskTree from './components/TaskTree.vue'
import TaskForm from './components/TaskForm.vue'
import { validateAll } from './utils/validator.js'
import { invoke } from "@tauri-apps/api/core";


// 任务类型定义
const taskTypes = ref({})

// 加载 task_types.json（只通过 Rust 命令）
async function loadTaskTypes() {
  try {
    const jsonStr = await invoke("get_task_types")
    taskTypes.value = JSON.parse(jsonStr)
    console.log("✅ 任务类型加载成功:", taskTypes.value)
  } catch (err) {
    console.error("❌ 加载任务类型失败:", err)
    alert("加载任务类型失败: " + err)
  }
}

onMounted(() => {
  loadTaskTypes()
})

// 任务树数据
const tasks = ref([
  {
    uuid: 'G1',
    name: '主线任务',
    type: 'serial_group',
    status: 0,
    children: []
  }
])

// 当前选中任务
const selectedTask = ref(null)

/** 导入 JSON 文件 */
function onImport(fileContent) {
  try {
    tasks.value = JSON.parse(fileContent).tasks || []
  } catch (e) {
    alert('❌ JSON 解析失败: ' + e.message)
  }
}

/** 导出 JSON 文件 */
function onExport() {
  const blob = new Blob(
    [JSON.stringify({ tasks: tasks.value }, null, 2)],
    { type: 'application/json' }
  )
  const a = document.createElement('a')
  a.href = URL.createObjectURL(blob)
  a.download = 'tasks.json'
  a.click()
}

/** 校验 JSON */
function onValidate() {
  const report = validateAll(tasks.value, taskTypes.value)
  alert(report.ok ? '✅ 校验通过' : '❌ 校验失败:\n' + report.errors.join('\n'))
}

/** 添加根任务（来自 Toolbar 弹窗选择） */
function onAddRootTask(type) {
  tasks.value.push({
    uuid: 'N' + Date.now(),
    name: taskTypes.value[type].label,
    type,
    status: 0,
    params: {},
    children: taskTypes.value[type].group ? [] : undefined
  })
}
</script>
