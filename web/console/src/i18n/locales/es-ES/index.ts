import type { Messages } from '@/i18n'
import core from './core'
import admin from './admin'

/** All the namespaces of this locale, merged: `core` (shell, login, devices, ...) and `admin` (users, groups, ...). */
const messages: Messages = { ...core, ...admin }
export default messages
