<script setup>
    import { invoke } from '@tauri-apps/api/core';
    import { nextTick, ref, onMounted, watch } from 'vue';

    const props = defineProps(['task_value']);
    const isChecked = ref(false);
    const isEditing = ref(false);
    const taskInput = ref(null);
    const taskText = ref("");
    const date = ref("");

    function statusToBoolean(status) {
        if (status == "DONE") {
            return true;
        } else if (status == "TODO") {
            return false;
        } else {
            return false;
        }
    }

    async function handleChange() {
        if (isChecked.value) {
            await invoke('mark_as_complete', { id: props.task_value.id })
                .catch((e) => console.log(e));
        } else {
            await invoke('mark_as_incomplete', { id: props.task_value.id })
                .catch((e) => console.log(e));
        }
    }

    async function handleEdit() {
        isEditing.value = true; 
        await nextTick(); 
        taskInput.value.focus();
        taskInput.value.select(); 
    }

    async function saveEdit() {
        await invoke('edit_task_text', {
            id: props.task_value.id,
            msg: taskText.value
        }).catch((e) => console.log(e)) 
        isEditing.value = false;
    }

    onMounted(() => {
        isChecked.value = statusToBoolean(props.task_value.status);
        taskText.value = props.task_value.text;
        date.value = props.task_value.date;
    });

    watch(() => props.task_value, (newState) => {
        isChecked.value = statusToBoolean(newState.status);
        taskText.value = newState.text;
        date.value = newState.date;
    });
</script>

<template>
    <li class="task-list-item">
        <div class="task">
            <label>
                <input 
                    type="checkbox" 
                    class="task-checkbox"
                    v-model="isChecked"
                    @change="handleChange()"
                />
                <input
                    ref="taskInput"
                    type="text"
                    v-model="taskText"
                    :disabled="!isEditing"
                    @blur="() => { isEditing = false }"
                    @keydown.enter="saveEdit"
                >
            </label>
            <div class="task-control">
                <div>{{ date }}</div>
                <button
                    @click="async () => {
                        await invoke('delete_task', { id: props.task_value.id })
                    }"
                >
                    <img class="delete" src="../assets/delete.svg" alt="delete">
                </button>
                <button
                    @click="handleEdit"
                    :disabled="isEditing"
                >
                    <img class="edit" src="../assets/edit.svg" alt="edit">
                </button>
            </div>
        </div>
    </li>
</template>