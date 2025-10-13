<![CDATA[
<template>
  <div class="card">
    <div v-if="$slots.header" class="card-header">
      <slot name="header"></slot>
    </div>
    <div class="card-body">
      <slot></slot>
    </div>
    <div v-if="$slots.footer" class="card-footer">
      <slot name="footer"></slot>
    </div>
  </div>
</template>

<style scoped>
.card {
  background-color: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sm);
  overflow: hidden;
  transition: all var(--transition-base);
  display: flex;
  flex-direction: column;
}

.card:hover {
  box-shadow: var(--shadow-md);
  transform: translateY(-2px);
}

.card-header {
  padding: var(--space-4) var(--space-5);
  border-bottom: 1px solid var(--border-color);
  font-weight: var(--font-semibold);
  font-size: var(--font-lg);
}

.card-body {
  padding: var(--space-5);
  flex: 1;
}

.card-footer {
  padding: var(--space-4) var(--space-5);
  border-top: 1px solid var(--border-color);
  background-color: var(--gray-50);
}
</style>
]]>