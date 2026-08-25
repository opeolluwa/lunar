<script setup lang="ts">
import { kButton, kNotification } from "konsta/vue";

definePageMeta({ name: "Notifications" });

const opened = ref(false);
</script>

<template>
  <div>
    <kNotification
      :opened="opened"
      title="Konsta UI"
      title-right-text="now"
      subtitle="This is a subtitle"
      text="This is a simple notification message"
    >
      <template #icon>
        <demo-icon />
      </template>
    </kNotification>

    <kButton outline @click="opened = !opened">Outline</kButton>
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Blanditiis
    doloremque delectus veritatis soluta praesentium amet similique saepe, quae
    itaque vel beatae, recusandae molestias perspiciatis quos exercitationem
    dolorum sapiente nulla tempora? Lorem ipsum dolor sit amet consectetur
    adipisicing elit. Blanditiis doloremque delectus veritatis soluta
    praesentium amet similique saepe, quae itaque vel beatae, recusandae
    molestias perspiciatis quos exercitationem dolorum sapiente nulla tempora?
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Blanditiis
    doloremque delectus veritatis soluta praesentium amet similique saepe, quae
    itaque vel beatae, recusandae molestias perspiciatis quos exercitationem
    dolorum sapiente nulla tempora? Lorem ipsum dolor sit amet consectetur
    adipisicing elit. Blanditiis doloremque delectus veritatis soluta
    praesentium amet similique saepe, quae itaque vel beatae, recusandae
    molestias perspiciatis quos e itaque vel beatae, recusandae molestias
    perspiciatis quos exercitationem dolorum sapiente nulla tempora? Lorem ipsum
    dolor sit amet consectetur adipisicing elit. Blanditiis doloremque delectus
    veritatis soluta praesentium amet similique saepe, quae itaque vel beatae,
    recusandae molestias perspiciatis quos exercitationem dolorum sapiente nulla
    tempora? Lorem ipsum dolor sit amet consectetur adipisicing elit. Blanditiis
    doloremque delectus veritatis soluta praesentium amet similique saepe, quae
    itaque vel beatae, recusandae molestias perspiciatis quos exercitationem
    dolorum sapiente nulla tempora? Lorem ipsum dolor sit amet consectetur
    adipisicing elit. Blanditiis doloremque delectus veritatis soluta
    praesentium amet similique saepe, quae itaque vel beatae, recusandae
    molestias perspiciatis quos e itaque vel beatae, recusandae molestias
    perspiciatis quos exercitationem dolorum sapiente nulla tempora? Lorem ipsum
    dolor sit amet consectetur adipisicing elit. Blanditiis doloremque delectus
    veritatis soluta praesentium amet similique saepe, quae itaque vel beatae,
    recusandae molestias perspiciatis quos exercitationem dolorum sapiente nulla
    tempora? Lorem ipsum dolor sit amet consectetur adipisicing elit. Blanditiis
    doloremque delectus veritatis soluta praesentium amet similique saepe, quae
    itaque vel beatae, recusandae molestias perspiciatis quos exercitationem
    dolorum sapiente nulla tempora? Lorem ipsum dolor sit amet consectetur
    adipisicing elit. Blanditiis doloremque delectus veritatis soluta
    praesentium amet similique saepe, quae itaque vel beatae, recusandae
    molestias perspiciatis quos e itaque vel beatae, recusandae molestias
    perspiciatis quos exercitationem dolorum sapiente nulla tempora? Lorem ipsum
    dolor sit amet consectetur adipisicing elit. Blanditiis doloremque delectus
    veritatis soluta praesentium amet similique saepe, quae itaque vel beatae,
    recusandae molestias perspiciatis quos exercitationem dolorum sapiente nulla
    tempora? Lorem ipsum dolor sit amet consectetur adipisicing elit. Blanditiis
    doloremque delectus veritatis soluta praesentium amet similique saepe, quae
    itaque vel beatae, recusandae molestias perspiciatis quos exercitationem
    dolorum sapiente nulla tempora? Lorem ipsum dolor sit amet consectetur
    adipisicing elit. Blanditiis doloremque delectus veritatis soluta
    praesentium amet similique saepe, quae itaque vel beatae, recusandae
    molestias perspiciatis quos e itaque vel beatae, recusandae molestias
    perspiciatis quos exercitationem dolorum sapiente nulla tempora? Lorem ipsum
    dolor sit amet consectetur adipisicing elit. Blanditiis doloremque delectus
    veritatis soluta praesentium amet similique saepe, quae itaque vel beatae,
    recusandae molestias perspiciatis quos exercitationem dolorum sapiente nulla
    tempora? Lorem ipsum dolor sit amet consectetur adipisicing elit. Blanditiis
    doloremque delectus veritatis soluta praesentium amet similique saepe, quae
    itaque vel beatae, recusandae molestias perspiciatis quos exercitationem
    dolorum sapiente nulla tempora? Lorem ipsum dolor sit amet consectetur
    adipisicing elit. Blanditiis doloremque delectus veritatis soluta
    praesentium amet similique saepe, quae itaque vel beatae, recusandae
    molestias perspiciatis quos e itaque vel beatae, recusandae molestias
    perspiciatis quos exercitationem dolorum sapiente nulla tempora? Lorem ipsum
    dolor sit amet consectetur adipisicing elit. Blanditiis doloremque delectus
    veritatis soluta praesentium amet similique saepe, quae itaque vel beatae,
    recusandae molestias perspiciatis quos exercitationem dolorum sapiente nulla
    tempora? Lorem ipsum dolor sit amet consectetur adipisicing elit. Blanditiis
    doloremque delectus veritatis soluta praesentium amet similique saepe, quae
    itaque vel beatae, recusandae molestias perspiciatis quos exercitationem
    dolorum sapiente nulla tempora? Lorem ipsum dolor sit amet consectetur
    adipisicing elit. Blanditiis doloremque delectus veritatis soluta
    praesentium amet similique saepe, quae itaque vel beatae, recusandae
    molestias perspiciatis quos exercitationem dolorum sapiente nulla tempora?
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Blanditiis
    doloremque delectus veritatis soluta praesentium amet similique saepe, quae
    itaque vel beatae, recusandae molestias perspiciatis quos exercitationem
    dolorum sapiente nulla tempora? Lorem ipsum dolor sit amet consectetur
    adipisicing elit. Blanditiis doloremque delectus veritatis soluta
    praesentium amet similique saepe, quae itaque vel beatae, recusandae
    molestias perspiciatis quos exercitationem dolorum sapiente nulla tempora?
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Blanditiis
    doloremque delectus veritatis soluta praesentium amet similique saepe, quae
    itaque vel beatae, recusandae molestias perspiciatis quos exercitationem
    dolorum sapiente nulla tempora? Lorem ipsum dolor sit amet consectetur
    adipisicing elit. Blanditiis doloremque delectus veritatis soluta
    praesentium amet similique saepe, quae itaque vel beatae, recusandae
    molestias perspiciatis quos exercitationem dolorum sapiente nulla tempora?
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Blanditiis
    doloremque delectus veritatis soluta praesentium amet similique saepe, quae
    itaque vel beatae, recusandae molestias perspiciatis quos exercitationem
    dolorum sapiente nulla tempora? Lorem ipsum dolor sit amet consectetur
    adipisicing elit. Blanditiis doloremque delectus veritatis soluta
    praesentium amet similique saepe, quae itaque vel beatae, recusandae
    molestias perspiciatis quos exercitationem dolorum sapiente nulla tempora?
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Blanditiis
    doloremque delectus veritatis soluta praesentium amet similique saepe, quae
    itaque vel beatae, recusandae molestias perspiciatis quos exercitationem
    dolorum sapiente nulla tempora? Lorem ipsum dolor sit amet consectetur
    adipisicing elit. Blanditiis doloremque delectus veritatis soluta
    praesentium amet similique saepe, quae itaque vel beatae, recusandae
    molestias perspiciatis quos exercitationem dolorum sapiente nulla tempora?
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Blanditiis
    doloremque delectus veritatis soluta praesentium amet similique saepe, quae
    itaque vel beatae, recusandae molestias perspiciatis quos exercitationem
    dolorum sapiente nulla tempora? Lorem ipsum dolor sit amet consectetur
    adipisicing elit. Blanditiis doloremque delectus veritatis soluta
    praesentium amet similique saepe, quae itaque vel beatae, recusandae
    molestias perspiciatis quos exercitationem dolorum sapiente nulla tempora?
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Blanditiis
    doloremque delectus veritatis soluta praesentium amet similique saepe, quae
    itaque vel beatae, recusandae molestias perspiciatis quos exercitationem
    dolorum sapiente nulla tempora? Lorem ipsum dolor sit amet consectetur
    adipisicing elit. Blanditiis doloremque delectus veritatis soluta
    praesentium amet similique saepe, quae itaque vel beatae, recusandae
    molestias perspiciatis quos exercitationem dolorum sapiente nulla tempora?
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Blanditiis
    doloremque delectus veritatis soluta praesentium amet similique saepe, quae
    itaque vel beatae, recusandae molestias perspiciatis quos exercitationem
    dolorum sapiente nulla tempora? Lorem ipsum dolor sit amet consectetur
    adipisicing elit. Blanditiis doloremque delectus veritatis soluta
    praesentium amet similique saepe, quae itaque vel beatae, recusandae
    molestias perspiciatis quos exercitationem dolorum sapiente nulla tempora?
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Blanditiis
    doloremque delectus veritatis soluta praesentium amet similique saepe, quae
    itaque vel beatae, recusandae molestias perspiciatis quos exercitationem
    dolorum sapiente nulla tempora?
  </div>
</template>
