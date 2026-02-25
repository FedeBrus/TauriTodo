<script setup>
import { invoke } from "@tauri-apps/api/core";
import { nextTick, ref, onMounted, onUnmounted, watch } from "vue";

const props = defineProps([
    "task_text",
    "task_status",
    "task_expiration",
    "task_date",
    "task_id",
]);
const isEditing = ref(false);
const taskContainer = ref(null);

const taskInput = ref(null);
const expirationInput = ref(null);

function statusToBoolean(status) {
    if (status == "Done") {
        return true;
    } else {
        return false;
    }
}

async function handleChange() {
    await invoke("toggle_status", { id: props.task_id }).catch((e) =>
        console.log(e),
    );
}

async function handleEdit() {
    isEditing.value = true;
    await nextTick();
    taskInput.value.focus();
    taskInput.value.select();
}

async function saveEdit() {
    await invoke("edit_task", {
        id: props.task_id,
        text: taskInput.value.value,
        expiration: expirationInput.value.value,
    }).catch((e) => console.log(e));
    isEditing.value = false;
}

onMounted(() => {
    window.addEventListener("mousedown", handleClickOutside);
});

onUnmounted(() => {
    window.removeEventListener("mousedown", handleClickOutside);
});

function handleClickOutside(event) {
    if (taskContainer.value.contains(event.target)) return;
    saveEdit();
}
</script>

<template>
    <li class="task-list-item">
        <div class="task" ref="taskContainer">
            <label>
                <input
                    ref="checkbox"
                    type="checkbox"
                    class="task-checkbox"
                    :checked="statusToBoolean(props.task_status)"
                    @click="handleChange()"
                />
                <input
                    ref="taskInput"
                    type="text"
                    :value="props.task_text"
                    :disabled="!isEditing"
                />
            </label>
            <label class="due">
                Due to:
                <input
                    ref="expirationInput"
                    type="date"
                    :value="props.task_expiration"
                    :disabled="!isEditing"
                    @change="
                        () => {
                            expirationInput.focus();
                        }
                    "
                />
            </label>
            <input type="date" :value="props.task_date" disabled="true" />
            <div class="task-control">
                <button
                    @click="
                        invoke('delete_task', {
                            id: props.task_id,
                        })
                    "
                >
                    <img
                        class="delete"
                        src="../assets/delete.svg"
                        alt="delete"
                    />
                </button>
                <button @click="handleEdit" v-if="!isEditing">
                    <img class="edit" src="../assets/edit.svg" alt="edit" />
                </button>
                <button class="confirm" @click="saveEdit" v-if="isEditing">
                    <img src="../assets/confirm.svg" alt="confirm" />
                </button>
            </div>
        </div>
    </li>
</template>
