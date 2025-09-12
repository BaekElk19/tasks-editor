<template>
  <div v-if="task">
    <h3>{{ task.name }} ({{ task.type }})</h3>
    <el-form
      ref="formRef"
      :model="task.params"
      label-width="120px"
      :rules="rules"
      status-icon
    >
      <el-form-item label="UUID">
        <el-input v-model="task.uuid" />
      </el-form-item>
      <el-form-item label="名称">
        <el-input v-model="task.name" />
      </el-form-item>

      <template v-for="f in taskTypes[task.type]?.fields" :key="f.key">
        <el-form-item
          :label="f.label"
          :prop="f.key"
          :required="f.required"
        >
          <component
            :is="getComponent(f)"
            v-model="task.params[f.key]"
            :placeholder="f.help || (f.required ? '必填' : '可选')"
            :options="f.values"
            clearable
          />
        </el-form-item>
      </template>
    </el-form>
  </div>
  <div v-else>
    <el-empty description="请选择任务" />
  </div>
</template>

<script setup>
import { ref, watch, computed } from "vue"

const props = defineProps({ task: Object, taskTypes: Object })

const formRef = ref(null)

/** 自动生成校验规则 */
const rules = computed(() => {
  if (!props.task || !props.taskTypes[props.task.type]) return {}
  const r = {}
  for (const f of props.taskTypes[props.task.type].fields || []) {
    if (f.required) {
      r[f.key] = [
        { required: true, message: `${f.label}为必填项`, trigger: "blur" }
      ]
    }
  }
  return r
})

/** 切换任务时自动校验 */
watch(
  () => props.task,
  (newTask) => {
    if (newTask && formRef.value) {
      formRef.value.validate().catch(() => {})
    }
  },
  { immediate: true }
)

function getComponent(field) {
  if (field.kind === "number") return "el-input-number"
  if (field.kind === "enum") return "el-select"
  if (field.kind === "boolean") return "el-switch"
  return "el-input"
}
</script>
