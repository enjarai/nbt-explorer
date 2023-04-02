<template>
    <div class="tag">
        <TagIcon :type="type"/>
        <span v-if="name" class="tag-name">{{name}}</span>

        <div v-if="simpleTag" class="simple-tag">
            <span class="tag-value">{{simpleTag.value}}</span>
        </div>
        <div v-else-if="listTag">
            <TagComp v-for="(item, i) in listTag.value" :key="i" :tag="item"/>
        </div>
        <div v-else-if="compoundTag">
            <TagComp v-for="(item, i) in compoundTag.value" :key="i" :tag="item" :name="i"/>
        </div>
    </div>
</template>

<script setup lang="ts">
import { Tag, isSimpleTag, isListTag, isCompoundTag } from "../types";
import TagIcon from "./TagIcon.vue";

const props = defineProps<{
    tag: Tag;
    name?: string;
}>();

const tag = props.tag;
const type = props.tag.type;
const name = props.name;

const simpleTag = isSimpleTag(tag);
const listTag = isListTag(tag);
const compoundTag = isCompoundTag(tag);
</script>

<style scoped>
.tag {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    border-radius: 0.5rem;
    background-color: #f0f0f0;
}

.simple-tag {
    display: flex;
    align-items: center;
    gap: 0.5rem;
}
</style>