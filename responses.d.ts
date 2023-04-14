import { ItemTypes, Tag } from "./shared"

export type Date = {
    year: number,
    month: number,
    day: number,
    time: {
        hour: number
        minute: number
    } | null
}


/**
 * Items are the basic primitive
 */
export interface Item {
    id: number
    when: Date | null
    deadline: Date | null
    /**
     * If parent is null, it is in the inbox or
     * (it is in the top level of the sidebar AND it's {@link ItemTypes | Item Type} is Area or Project)
     */
    parent: number | null
    /**
     * Because this is recrusive, this data only goes so deep. It will be null if children are not fetched,
     * regardless of if there are actually any
     * 
     * The rules are based on {@link ItemTypes | Item Types}, and are as follows:
     * - Items and Sections always return children
     * - Areas and Projects only do at the top level
     */
    children: Item[] | null
    title: string
    /**
     * Markdown formatted
     */
    body: string | null
    type: ItemTypes,
    tags: Tag[]
}

type a = Item['children']