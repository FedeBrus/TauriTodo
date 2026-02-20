<script setup>
    const props = defineProps(['task_value']);
</script>

<template>
    <li class="task-list-item">
        <label class="form-control">
            <input type="checkbox" class="task-checkbox"/>
            {{ props.task_value.msg }} 
        </label>
    </li>
</template>