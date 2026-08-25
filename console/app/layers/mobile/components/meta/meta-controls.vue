<script lang="ts" setup>
import { ref } from "vue";
import { kSheet } from "konsta/vue";
import { useWorkspacesStore } from "@shared/stores/workspaces";

const { notify } = useAppNotification();

const workspaceStore = useWorkspacesStore();
const showDuplicateModal = ref(false);
const showTransferModal = ref(false);
const showDeleteConfirm = ref(false);

const targetWorkspaceId = ref("");
const isSubmitting = ref(false);

const sheetOpened = ref(false);

const currentWorkspaceId = computed(() => workspaceStore.activeWorkspaceId);
const emit = defineEmits([
  "transferRecord",
  "duplicateRecord",
  "editRecord",
  "deleteRecord",
]);
const props = defineProps({
  itemName: {
    type: String,
    required: false,
    default: "",
  },
  showEdit: {
    type: Boolean,
    default: true,
  },
  showDuplicate: {
    type: Boolean,
    default: true,
  },
  showTransfer: {
    type: Boolean,
    default: true,
  },
});

const controls = computed(() => [
  [
    ...(props.showDuplicate
      ? [
          {
            label: "Duplicate",
            icon: "i-lucide-copy",
            class: "text-gray-700 dark:text-gray-300",
            onSelect: () => {
              showDuplicateModal.value = true;
            },
          },
        ]
      : []),
    ...(props.showTransfer
      ? [
          {
            label: "Transfer to workspace",
            icon: "i-lucide-arrow-right-left",
            class: "text-gray-700 dark:text-gray-300",
            onSelect: () => {
              showTransferModal.value = true;
            },
          },
        ]
      : []),
  ],
  [
    ...(props.showEdit
      ? [
          {
            label: "Edit",
            icon: "heroicons:pencil-square",
            class: "text-amber-500 dark:text-amber-400 font-medium",
            onSelect: () => emit("editRecord"),
          },
        ]
      : []),
    {
      label: "Delete",
      icon: "heroicons:trash",
      class: "text-red-500 dark:text-red-400 font-medium",
      onSelect: () => {
        showDeleteConfirm.value = true;
      },
    },
  ],
]);

function openSheet() {
  sheetOpened.value = true;
}

function closeSheet() {
  sheetOpened.value = false;
}

function handleItemSelect(onSelect: () => void) {
  closeSheet();
  onSelect();
}

const handleDuplicate = async () => {
  if (!targetWorkspaceId.value || isSubmitting.value) return;

  isSubmitting.value = true;

  try {
    emit("duplicateRecord", targetWorkspaceId.value);
    showDuplicateModal.value = false;
    targetWorkspaceId.value = "";
    notify({ message: `${props.itemName} duplicated successfully` });
  } finally {
    isSubmitting.value = false;
  }
};

const handleTransfer = async () => {
  if (!targetWorkspaceId.value || isSubmitting.value) return;

  isSubmitting.value = true;

  try {
    emit("transferRecord", targetWorkspaceId.value);
    showTransferModal.value = false;
    targetWorkspaceId.value = "";
    notify({ message: `${props.itemName} transferred successfully` });
  } finally {
    isSubmitting.value = false;
  }
};

const handleDelete = () => {
  emit("deleteRecord");
  showDeleteConfirm.value = false;
};

watch(showDuplicateModal, (open) => {
  if (open) targetWorkspaceId.value = "";
});

watch(showTransferModal, (open) => {
  if (open) targetWorkspaceId.value = "";
});

const workspaces = computed(() => [
  ...workspaceStore.workspaces
    .filter((w): w is Workspace => !!w)
    .map((w) => {
      const isActive = w.identifier === currentWorkspaceId.value;
      return {
        label: w.name,
        value: w.identifier,

        icon: isActive
          ? "heroicons:check-circle-solid"
          : "heroicons:check-circle",
        class: isActive ? "font-semibold text-primary-500" : "",
        disabled: w.identifier === currentWorkspaceId.value,
      };
    }),
]);
</script>

<template>
  <div>
    <button
      class="p-1 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors duration-150 focus:outline-none"
      @click="openSheet"
    >
      <UIcon
        name="heroicons:ellipsis-vertical"
        class="size-5 text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 shrink-0 transition-colors"
      />
    </button>

    <kSheet :opened="sheetOpened" @backdropclick="closeSheet">
      <div
        class="bg-white dark:bg-app-dark-800 rounded-t-2xl px-4 pb-8 pt-3"
        style="padding-bottom: env(safe-area-inset-bottom, 8px)"
      >
        <div class="mx-auto mb-3 h-1 w-10 rounded-full bg-gray-300 dark:bg-gray-600" />

        <template v-for="(group, gi) in controls" :key="gi">
          <div v-if="group.length" class="py-1">
            <button
              v-for="(item, ii) in group"
              :key="ii"
              :class="[
                'flex items-center gap-3 w-full px-4 py-3 text-sm rounded-xl transition-colors active:bg-gray-100 dark:active:bg-gray-700',
                item.class,
              ]"
              @click="handleItemSelect(item.onSelect)"
            >
              <UIcon :name="item.icon" class="size-5 shrink-0" />
              <span>{{ item.label }}</span>
            </button>
          </div>
          <div
            v-if="gi < controls.length - 1 && group.length"
            class="my-1 border-t border-gray-100 dark:border-gray-800"
          />
        </template>
      </div>
    </kSheet>

    <UModal v-model:open="showDuplicateModal">
      <template #content>
        <div class="px-6 pt-6 pb-2 flex flex-col gap-1">
          <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
            Duplicate {{ itemName }}
          </h2>
          <p class="text-sm text-gray-500 dark:text-gray-400">
            Create a duplicate of this {{ itemName }} in another workspace.
          </p>

          <AppSelect
            v-model="targetWorkspaceId"
            placeholder="Select target workspace"
            :items="workspaces"
            class="w-full mt-8"
          />
          <AppButton
            class="mt-2 w-fit ml-auto"
            :disabled="!targetWorkspaceId || isSubmitting"
            @click="handleDuplicate"
          >
            {{ isSubmitting ? "Processing..." : "Duplicate" }}
          </AppButton>
        </div>
      </template>
    </UModal>

    <UModal v-model:open="showTransferModal">
      <template #content>
        <div class="px-6 pt-6 pb-2 flex flex-col gap-1">
          <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
            Transfer {{ itemName }}
          </h2>
          <p class="text-sm text-gray-500 dark:text-gray-400">
            Transfer this {{ itemName }} to another workspace?
          </p>

          <AppSelect
            v-model="targetWorkspaceId"
            placeholder="Select target workspace"
            label="Select target workspace"
            :items="workspaces"
            class="w-full mt-8"
          />
          <AppButton
            class="mt-2 w-fit ml-auto"
            :disabled="!targetWorkspaceId || isSubmitting"
            @click="handleTransfer"
          >
            {{ isSubmitting ? "Processing..." : "Transfer" }}
          </AppButton>
        </div>
      </template>
    </UModal>

    <UModal v-model:open="showDeleteConfirm">
      <template #content>
        <div class="px-6 py-6 flex flex-col gap-5">
          <div class="flex items-start gap-4">
            <div
              class="shrink-0 size-10 rounded-full bg-red-100 dark:bg-red-950 flex items-center justify-center"
            >
              <UIcon
                name="heroicons:trash"
                class="size-5 text-red-600 dark:text-red-400"
              />
            </div>
            <div class="flex flex-col gap-1">
              <h2 class="text-base font-semibold text-gray-900 dark:text-white">
                Delete {{ itemName }}
              </h2>
              <p class="text-sm text-gray-500 dark:text-gray-400">
                Are you sure you want to delete this {{ itemName }}? This action
                cannot be undone.
              </p>
            </div>
          </div>

          <div class="flex justify-end gap-2">
            <UButton size="sm" variant="ghost" @click="showDeleteConfirm = false">
              Cancel
            </UButton>
            <UButton size="sm" color="error" @click="handleDelete">
              Delete
            </UButton>
          </div>
        </div>
      </template>
    </UModal>
  </div>
</template>
