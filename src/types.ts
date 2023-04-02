export type NumberTag = {
    type: "byte" | "short" | "int" | "long" | "float" | "double";
    value: number;
}
export type NumberArrayTag = {
    type: "byte_array" | "int_array" | "long_array";
    value: number[];
}
export type StringTag = {
    type: "string";
    value: string;
}
export type ListTag = {
    type: "list";
    value: Tag[];
}
export type CompoundTag = {
    type: "compound";
    value: Record<string, Tag>;
}

export type SimpleTag = NumberTag | StringTag;
export type Tag = NumberTag | StringTag | ListTag | CompoundTag | NumberArrayTag;

export const isSimpleTag = (tag: Tag): SimpleTag | null => {
    if (tag.type === "byte" || tag.type === "short" || tag.type === "int" || tag.type === "long" || tag.type === "float" || tag.type === "double" || tag.type === "string") {
        return tag;
    }
    return null;
}

export const isListTag = (tag: Tag): ListTag | null => {
    if (tag.type === "list") {
        return tag;
    }
    return null;
}

export const isNumberArrayTag = (tag: Tag): NumberArrayTag | null => {
    if (tag.type === "byte_array" || tag.type === "int_array" || tag.type === "long_array") {
        return tag;
    }
    return null;
}

export const isCompoundTag = (tag: Tag): CompoundTag | null => {
    if (tag.type === "compound") {
        return tag;
    }
    return null;
}