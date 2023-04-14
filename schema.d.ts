import {Generated} from 'kysely'
import { ItemTypes, Status } from './shared'

type Time = `${number}:${number}`
type Date = `${number}/${number}/${number}${` ${Time}` | undefined}`
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

interface User {
    id: Generated<number>
    uuid: Generated<number>
}

interface ItemTags {
    item: number
    tag: number
}

interface Tag {
    id: Generated<number>
    title: Generated<number>
}