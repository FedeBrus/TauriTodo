<script setup>
    import { ref, onMounted } from 'vue';

    const input = ref(null);
    const text = ref("");

    const emit = defineEmits(['lose-focus', 'save-task']);

    onMounted(() => {
        input.value.focus();
    });

    defineExpose({ focus });
</script>

<template>
    <li class="task-list-item">
        <input
            ref="input" 
            v-model="text" 
            type="text" 
            @blur="() => { emit('lose-focus'); }"
            @keydown.enter="() => { emit('save-task', text) }"
        >
    </li>
</template>