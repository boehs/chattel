import {Generated} from 'kysely'
import { ItemTypes, Status } from './shared'

type Date = `${number}/${number}/${number}`
type When = "Today" | "Evening" | "Someday" | Date | null

interface Item {
    id: Generated<number>
    owner: number
    when: When | null
    deadline: Date | null
    parent: number | null
    title: string
    body: string | null
    type: ItemTypes,
    status: Status
}

interface TimeBlocks {
    id: Generated<number>,
    item: number
    start: number,
    end: number
}

interface User {
    id: Generated<number>
    uuid: Generated<number>
    invites: boolean
}

interface ItemTags {
    item: number
    tag: number
}

interface Tag {
    id: Generated<number>
    title: Generated<number>
}

interface invites {
    code: string
    by: number
}

export interface Db {
    item: Item
    itmeBlocks: TimeBlocks,
    itemTags: ItemTypes,
    tag: Tag
    user: User
    invites: invites
}