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

// 任务类型定义
const taskTypes = ref({})

// 加载 task_types.json
onMounted(async () => {
  try {
    // 优先尝试读取 exe 同目录下的 task_types.json
    const txt = await readTextFile('task_types.json')
    taskTypes.value = JSON.parse(txt)
  } catch (e) {
    // 如果外部没有，就回退到内置的
    const res = await fetch('./task_types.json')
    taskTypes.value = await res.json()
  }
})

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
