import type { Messages } from '@/i18n'

/** Messages of the administration views (users, groups, address books). */
export default {
  "admin": {
    "update": "Actualizar",
    "fields": {
      "id": "Id",
      "alias": "Alias",
      "hostname": "Nombre de host",
      "login": "Inicio de sesión",
      "platform": "Plataforma",
      "username": "Nombre de usuario",
      "password": "Contraseña",
      "confirm": "Confirmar",
      "email": "Correo electrónico",
      "administrator": "Administrador",
      "group": "Grupo",
      "guid": "Guid",
      "name": "Nombre",
      "note": "Nota",
      "status": "Estado",
      "admin": "Admin",
      "user": "Usuario",
      "rights": "Permisos"
    },
    "users": {
      "add": "Añadir usuario",
      "edit": "Editar usuario",
      "active": "Activo",
      "inactive": "Inactivo",
      "activate": "Activar",
      "deactivate": "Desactivar",
      "copied": "¡Copiado!",
      "fillAll": "Por favor, rellene todos los campos",
      "mismatch": "La contraseña y su confirmación no coinciden",
      "addFailed": "No se pudo añadir el usuario",
      "updateFailed": "No se pudo actualizar el usuario",
      "activateFailed": "No se pudo activar al usuario {name}",
      "deactivateFailed": "No se pudo desactivar al usuario {name}",
      "confirmDelete": "¿Seguro que desea eliminar este usuario?",
      "deleted": "Usuario eliminado correctamente",
      "deleteFailed": "No se pudo eliminar el usuario"
    },
    "groups": {
      "add": "Añadir grupo",
      "update": "Actualizar grupo",
      "name": "Nombre del grupo",
      "confirmDelete": "¿Seguro que desea eliminar este grupo?",
      "canConnectTo": "Puede conectarse a",
      "canBeReachedBy": "Puede ser alcanzado por"
    },
    "ab": {
      "personal": "Libreta de direcciones personal",
      "createShared": "Crear una libreta de direcciones compartida",
      "addShared": "Añadir libreta de direcciones compartida",
      "sharedName": "Nombre de la libreta compartida",
      "update": "Actualizar libreta de direcciones",
      "name": "Nombre de la libreta de direcciones",
      "viewRules": "Ver reglas",
      "addRule": "Añadir regla",
      "edit": "Editar libreta de direcciones",
      "delete": "Eliminar libreta de direcciones",
      "confirmDelete": "¿Seguro que desea eliminar esta libreta de direcciones?",
      "rulesTitle": "Reglas",
      "addRuleFor": "Añadir regla para {ab}",
      "nullOption": "Ninguno",
      "ruleError": "El usuario debe estar vacío en una regla de grupo y el grupo vacío en una regla de usuario",
      "rights": {
        "none": "Ninguno",
        "read": "Lectura",
        "write": "Escritura",
        "full": "Control total"
      },
      "viewRights": {
        "none": "Ninguno",
        "read": "Lectura",
        "readWrite": "Lectura/escritura",
        "total": "Control total",
        "unknown": "Desconocido"
      }
    }
  }
} as Messages
