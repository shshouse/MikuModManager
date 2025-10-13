<![CDATA[
<template>
  <input
    :value="modelValue"
    @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
    class="form-input"
  />
</template>

<script setup lang="ts">
defineProps<{
  modelValue: string | number
}>()
defineEmits(['update:modelValue'])
</script>

<style scoped>
.form-input {
  width: 100%;
  padding: var(--space-3) var(--space-4);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-base);
  font-size: var(--font-base);
  transition: all var(--transition-base);
  background-color: var(--bg-input);
  color: var(--text-primary);
}

.form-input:focus {
  outline: none;
  border-color: var(--primary-color);
  box-shadow: 0 0 0 3px rgba(52, 152, 219, 0.2);
}
</style>
]]>