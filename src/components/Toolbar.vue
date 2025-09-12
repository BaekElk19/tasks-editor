<template>
  <div class="toolbar">
    <!-- 导入/导出/校验 -->
    <el-upload 
      action="" 
      :auto-upload="false" 
      :show-file-list="false" 
      accept=".json"
      :on-change="onFileChange">
      <el-button>📂 导入 JSON</el-button>
    </el-upload>
    <el-button type="primary" @click="$emit('export')">💾 导出</el-button>
    <el-button @click="$emit('validate')">✅ 校验</el-button>
    <el-button type="success" @click="openAddDialog">➕ 添加任务</el-button>

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
        <el-button type="primary" @click="confirmAdd">确认</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const props = defineProps({
  taskTypes: Object
})
const emit = defineEmits(['import','export','validate','addRootTask'])

const dialog = ref({
  visible: false,
  selected: ''
})

function onFileChange(file) {
  const reader = new FileReader()
  reader.onload = e => emit('import', e.target.result)
  reader.readAsText(file.raw)
}

function openAddDialog() {
  dialog.value.visible = true
  dialog.value.selected = ''
}

function confirmAdd() {
  if (!dialog.value.selected) return
  emit('addRootTask', dialog.value.selected)
  dialog.value.visible = false
}
</script>

<style scoped>
.toolbar {
  display: flex;
  gap: 8px;
  align-items: center;
}
</style>
