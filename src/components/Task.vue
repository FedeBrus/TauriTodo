<script setup>
    import { invoke } from '@tauri-apps/api/core';
    import { ref, onMounted, watch } from 'vue';

    const props = defineProps(['task_value']);
    const isChecked = ref(false);

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

    onMounted(() => {
        isChecked.value = statusToBoolean(props.task_value.status);
    });

    watch(() => props.task_value.status, (newStatus) => {
        isChecked.value = statusToBoolean(newStatus);
    });
</script>

<template>
    <li class="task-list-item">
        <div>
            <label class="form-control">
                <input 
                    type="checkbox" 
                    class="task-checkbox"
                    v-model="isChecked"
                    @change="handleChange()"
                />
                {{ props.task_value.msg }} 
            </label>
            <button
                @click="async () => {
                    await invoke('delete_task', { id: props.task_value.id })
                }"
            >
                Delete
            </button>
        </div>
    </li>
</template>