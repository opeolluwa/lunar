<script setup lang="ts">
import {
  kBlock,
  kBlockTitle,
  kButton,
  kActions,
  kActionsButton,
  kActionsLabel,
  kActionsGroup,
} from "konsta/vue";

definePageMeta({ name: "Reminders" });

const actionsOneOpened = ref(false);
const actionsTwoOpened = ref(false);
</script>

<template>
  <kBlock strong inset class="space-y-4">
    <p>
      Action Sheet is a slide-up pane for presenting the user with a set of
      alternatives for how to proceed with a given task.
    </p>
  </kBlock>
  <kBlockTitle>Open Action Sheet</kBlockTitle>
  <kBlock strong inset class="flex space-x-4 rtl:space-x-reverse">
    <kButton rounded @click="actionsOneOpened = true">One group</kButton>
    <kButton rounded @click="actionsTwoOpened = true">Two groups</kButton>
  </kBlock>
  <kActions
    :opened="actionsOneOpened"
    @backdropclick="actionsOneOpened = false"
  >
    <kActionsGroup>
      <kActionsLabel>Do something</kActionsLabel>
      <kActionsButton bold @click="actionsOneOpened = false"
        >Button 1</kActionsButton
      >
      <kActionsButton @click="actionsOneOpened = false"
        >Button 2</kActionsButton
      >
      <kActionsButton @click="actionsOneOpened = false">Cancel</kActionsButton>
    </kActionsGroup>
  </kActions>
  <kActions
    :opened="actionsTwoOpened"
    @backdropclick="actionsTwoOpened = false"
  >
    <kActionsGroup>
      <kActionsLabel>Do something</kActionsLabel>
      <kActionsButton bold @click="actionsTwoOpened = false"
        >Button 1</kActionsButton
      >
      <kActionsButton @click="actionsTwoOpened = false"
        >Button 2</kActionsButton
      >
    </kActionsGroup>
    <kActionsGroup>
      <kActionsButton @click="actionsTwoOpened = false">Cancel</kActionsButton>
    </kActionsGroup>
  </kActions>
</template>
