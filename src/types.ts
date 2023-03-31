export type Tag = {
    type: "byte" | "short" | "int" | "long" | "float" | "double";
    value: number;
} | {
    type: "byte_array" | "int_array" | "long_array";
    value: number[];
} | {
    type: "string";
    value: string;
} | {
    type: "list";
    value: Tag[];
} | {
    type: "compound";
    value: Record<string, Tag>;
}
