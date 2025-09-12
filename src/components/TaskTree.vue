<template>
  <el-tree
    :data="tasks"
    node-key="uuid"
    default-expand-all
    draggable
    highlight-current
    :props="{ label: 'name', children: 'children' }"
    :allow-drop="allowDrop"
    @node-click="handleClick"
    @node-drop="handleDrop"
    @node-drag-enter="handleDragEnter"
    @node-drag-leave="handleDragLeave"
  >
    <template #default="{ node, data }">
      <el-dropdown trigger="contextmenu" @command="cmd => handleCommand(cmd, data)">
        <div
          :class="['tree-node', highlightId === data.uuid ? 'highlight' : '']"
          @contextmenu.prevent
        >
          <span>{{ data.name }} ({{ data.type }})</span>
        </div>
        <template #dropdown>
          <el-dropdown-menu>
            <el-dropdown-item command="addTask">➕ 添加子任务</el-dropdown-item>
            <el-dropdown-item command="addSerial">➕ 添加串行组</el-dropdown-item>
            <el-dropdown-item command="addParallel">➕ 添加并行组</el-dropdown-item>
            <el-dropdown-item divided command="duplicate">📑 复制</el-dropdown-item>
            <el-dropdown-item divided command="delete" style="color:red">🗑 删除</el-dropdown-item>
          </el-dropdown-menu>
        </template>
      </el-dropdown>
    </template>
  </el-tree>

  <!-- 添加任务对话框 -->
  <el-dialog v-model="dialog.visible" title="选择任务类型" width="400px">
    <el-form>
      <el-form-item label="任务类型">
        <el-select v-model="dialog.selected" placeholder="请选择">
          <el-option
            v-for="(cfg, type) in taskTypes"
            :key="type"
            :label="cfg.label"
            :value="type"
          />
        </el-select>
      </el-form-item>
    </el-form>
    <template #footer>
      <el-button @click="dialog.visible = false">取消</el-button>
      <el-button type="primary" @click="confirmAddTask">确认</el-button>
    </template>
  </el-dialog>
</template>

<script setup>
import { ref } from 'vue'

const props = defineProps({
  tasks: Array,
  taskTypes: Object
})
const emit = defineEmits(['update:selected'])

const highlightId = ref(null)

// 对话框状态
const dialog = ref({
  visible: false,
  parent: null,
  selected: ''
})

function handleClick(node) {
  emit('update:selected', node)
}

function allowDrop(draggingNode, dropNode, type) {
  if (type === 'inner') {
    return dropNode.data.type === 'serial_group' || dropNode.data.type === 'parallel_group'
  }
  return true
}

function handleDrop(draggingNode, dropNode, type) {
  highlightId.value = null
  const dragged = draggingNode.data
  const dropped = dropNode.data
  removeNode(props.tasks, dragged.uuid)

  if (type === 'inner') {
    dropped.children = dropped.children || []
    dropped.children.push(dragged)
  } else if (type === 'before') {
    const parentArr = findParentArray(props.tasks, dropped.uuid)
    const idx = parentArr.findIndex(n => n.uuid === dropped.uuid)
    parentArr.splice(idx, 0, dragged)
  } else if (type === 'after') {
    const parentArr = findParentArray(props.tasks, dropped.uuid)
    const idx = parentArr.findIndex(n => n.uuid === dropped.uuid)
    parentArr.splice(idx + 1, 0, dragged)
  }
}

function handleDragEnter(draggingNode, dropNode) {
  if (dropNode.data.type === 'serial_group' || dropNode.data.type === 'parallel_group') {
    highlightId.value = dropNode.data.uuid
  }
}

function handleDragLeave(draggingNode, dropNode) {
  if (highlightId.value === dropNode.data.uuid) {
    highlightId.value = null
  }
}

/** 菜单命令 */
function handleCommand(cmd, node) {
  if (cmd === 'addTask') {
    dialog.value.visible = true
    dialog.value.parent = node
    dialog.value.selected = ''
  }
  if (cmd === 'addSerial') {
    node.children = node.children || []
    node.children.push({
      uuid: 'G' + Date.now(),
      name: '串行组',
      type: 'serial_group',
      status: 0,
      children: []
    })
  }
  if (cmd === 'addParallel') {
    node.children = node.children || []
    node.children.push({
      uuid: 'G' + Date.now(),
      name: '并行组',
      type: 'parallel_group',
      status: 0,
      children: []
    })
  }
  if (cmd === 'duplicate') {
    const copy = JSON.parse(JSON.stringify(node))
    copy.uuid = 'N' + Date.now()
    const parentArr = findParentArray(props.tasks, node.uuid)
    const idx = parentArr.findIndex(n => n.uuid === node.uuid)
    parentArr.splice(idx + 1, 0, copy)
  }
  if (cmd === 'delete') {
    removeNode(props.tasks, node.uuid)
  }
}

/** 确认添加子任务 */
function confirmAddTask() {
  if (!dialog.value.selected) return
  const parent = dialog.value.parent
  parent.children = parent.children || []
  parent.children.push({
    uuid: 'N' + Date.now(),
    name: props.taskTypes[dialog.value.selected].label,
    type: dialog.value.selected,
    status: 0,
    params: {},
    children: props.taskTypes[dialog.value.selected].group ? [] : undefined
  })
  dialog.value.visible = false
}

// ---------- 工具函数 ----------
function removeNode(arr, uuid) {
  for (let i = arr.length - 1; i >= 0; i--) {
    if (arr[i].uuid === uuid) {
      arr.splice(i, 1)
      return true
    }
    if (arr[i].children) {
      if (removeNode(arr[i].children, uuid)) return true
    }
  }
  return false
}

function findParentArray(arr, uuid) {
  for (const node of arr) {
    if (node.uuid === uuid) return arr
    if (node.children) {
      const res = findParentArray(node.children, uuid)
      if (res) return res
    }
  }
  return null
}
</script>

<style scoped>
.tree-node {
  padding: 4px 8px;
  border-radius: 4px;
}
.tree-node.highlight {
  background-color: #e0f2fe;
  border: 1px dashed #3b82f6;
}
</style>
