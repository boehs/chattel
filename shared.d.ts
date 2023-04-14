export enum ItemTypes {
    Area,
    Project,
    Section,
    Item
}

export enum Status {
    ToDo,
    Completed,
    Canceled,
    Trashed
}

export type Tag = [number, string]