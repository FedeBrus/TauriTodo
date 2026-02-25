<script setup>
import { invoke } from "@tauri-apps/api/core";
import { nextTick, ref, onMounted, onUnmounted, watch } from "vue";

const props = defineProps(["text"]);
const isEditing = ref(false);
const taskContainer = ref(null);

const taskInput = ref(null);
const expirationInput = ref(null);

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
    if (!isChecked.value) {
        await invoke("mark_as_complete", { id: props.task_id }).catch((e) =>
            console.log(e),
        );
    } else {
        await invoke("mark_as_incomplete", { id: props.task_id }).catch((e) =>
            console.log(e),
        );
    }
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
        // text: taskText.value,
        // expiration: taskExpiration.value,
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
    // mettere tutti i prop
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
                    :checked="statusToBoolean(props.task_value.status)"
                    @click="handleChange()"
                />
                <input
                    ref="taskInput"
                    type="text"
                    text="props.task_value.text"
                    :disabled="!isEditing"
                />
            </label>
            <label class="due">
                Due to:
                <input
                    ref="expirationInput"
                    type="date"
                    :disabled="!isEditing"
                    @change="
                        () => {
                            expirationInput.focus();
                        }
                    "
                />
            </label>
            <div class="task-control">
                <input type="date" disabled="true" />
                <button
                    @click="
                        async () => {
                            await invoke('delete_task', {
                                id: props.task_value.id,
                            });
                        }
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
                <button
                    class="confirm"
                    @click="
                        () => {
                            saveEdit();
                        }
                    "
                    v-if="isEditing"
                >
                    <img src="../assets/confirm.svg" alt="confirm" />
                </button>
            </div>
        </div>
    </li>
</template>
