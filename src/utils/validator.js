export function validateAll(tasks, taskTypes) {
  const errors = []

  function validateTask(task) {
    const typeDef = taskTypes[task.type]
    if (!typeDef) {
      errors.push(`未知任务类型: ${task.type} (uuid=${task.uuid})`)
      return
    }

    // 检查必填参数
    if (typeDef.fields) {
      typeDef.fields.forEach(f => {
        if (f.required) {
          const val = task.params?.[f.key]
          const isEmpty =
            val === undefined ||
            val === null ||
            (typeof val === "string" && val.trim() === "")

          if (isEmpty) {
            errors.push(
              `任务(${task.name}, uuid=${task.uuid}) 缺少必填参数: ${f.label}`
            )
          }
        }
      })
    }

    // 递归校验子任务
    if (task.children) {
      task.children.forEach(validateTask)
    }
  }

  tasks.forEach(validateTask)

  return { ok: errors.length === 0, errors }
}
