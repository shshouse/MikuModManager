<![CDATA[
<template>
  <button :class="buttonClass" :disabled="disabled">
    <slot></slot>
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  variant?: 'primary' | 'secondary' | 'danger' | 'ghost'
  size?: 'sm' | 'md' | 'lg'
  disabled?: boolean
}>()

const buttonClass = computed(() => [
  'btn',
  `btn-${props.variant || 'primary'}`,
  `btn-${props.size || 'md'}`,
  { 'disabled': props.disabled }
])
</script>

<style scoped>
.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-base);
  font-weight: var(--font-semibold);
  cursor: pointer;
  transition: all var(--transition-base);
  border: 1px solid transparent;
  white-space: nowrap;
}

.btn:disabled {
  cursor: not-allowed;
  opacity: 0.6;
}

/* Sizes */
.btn-sm {
  padding: var(--space-2) var(--space-3);
  font-size: var(--font-sm);
}
.btn-md {
  padding: var(--space-3) var(--space-5);
  font-size: var(--font-base);
}
.btn-lg {
  padding: var(--space-4) var(--space-6);
  font-size: var(--font-lg);
}

/* Variants */
.btn-primary {
  background-color: var(--primary-color);
  color: var(--text-white);
  border-color: var(--primary-color);
}
.btn-primary:hover:not(:disabled) {
  background-color: var(--primary-dark);
  border-color: var(--primary-dark);
  transform: translateY(-1px);
  box-shadow: var(--shadow-sm);
}

.btn-secondary {
  background-color: var(--gray-200);
  color: var(--text-primary);
  border-color: var(--border-color);
}
.btn-secondary:hover:not(:disabled) {
  background-color: var(--gray-300);
  border-color: var(--gray-400);
}

.btn-danger {
  background-color: var(--error-color);
  color: var(--text-white);
  border-color: var(--error-color);
}
.btn-danger:hover:not(:disabled) {
  background-color: #c0392b;
  border-color: #c0392b;
}

.btn-ghost {
  background-color: transparent;
  color: var(--text-primary);
  border-color: transparent;
}
.btn-ghost:hover:not(:disabled) {
  background-color: var(--gray-100);
}
</style>
]]>