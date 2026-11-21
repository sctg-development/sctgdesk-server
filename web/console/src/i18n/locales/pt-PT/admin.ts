import type { Messages } from '@/i18n'

/** Messages of the administration views (users, groups, address books). */
export default {
  "admin": {
    "update": "Atualizar",
    "fields": {
      "id": "Id",
      "alias": "Alias",
      "hostname": "Nome do anfitrião",
      "login": "Início de sessão",
      "platform": "Plataforma",
      "username": "Nome de utilizador",
      "password": "Palavra-passe",
      "confirm": "Confirmar",
      "email": "E-mail",
      "administrator": "Administrador",
      "group": "Grupo",
      "guid": "Guid",
      "name": "Nome",
      "note": "Nota",
      "status": "Estado",
      "admin": "Admin",
      "user": "Utilizador",
      "rights": "Permissões"
    },
    "users": {
      "add": "Adicionar utilizador",
      "edit": "Editar utilizador",
      "active": "Ativo",
      "inactive": "Inativo",
      "activate": "Ativar",
      "deactivate": "Desativar",
      "copied": "Copiado!",
      "fillAll": "Preencha todos os campos",
      "mismatch": "A palavra-passe e a confirmação não coincidem",
      "addFailed": "Não foi possível adicionar o utilizador",
      "updateFailed": "Não foi possível atualizar o utilizador",
      "activateFailed": "Não foi possível ativar o utilizador {name}",
      "deactivateFailed": "Não foi possível desativar o utilizador {name}",
      "confirmDelete": "Tem a certeza de que pretende eliminar este utilizador?",
      "deleted": "Utilizador eliminado com sucesso",
      "deleteFailed": "Não foi possível eliminar o utilizador"
    },
    "groups": {
      "add": "Adicionar grupo",
      "update": "Atualizar grupo",
      "name": "Nome do grupo",
      "confirmDelete": "Tem a certeza de que pretende eliminar este grupo?",
      "canConnectTo": "Pode ligar-se a",
      "canBeReachedBy": "Pode ser contactado por"
    },
    "ab": {
      "personal": "Livro de endereços pessoal",
      "createShared": "Criar um livro de endereços partilhado",
      "addShared": "Adicionar livro de endereços partilhado",
      "sharedName": "Nome do livro de endereços partilhado",
      "update": "Atualizar livro de endereços",
      "name": "Nome do livro de endereços",
      "viewRules": "Ver regras",
      "addRule": "Adicionar regra",
      "edit": "Editar livro de endereços",
      "delete": "Eliminar livro de endereços",
      "confirmDelete": "Tem a certeza de que pretende eliminar este livro de endereços?",
      "rulesTitle": "Regras",
      "addRuleFor": "Adicionar regra para {ab}",
      "nullOption": "Nenhum",
      "ruleError": "O utilizador deve estar vazio numa regra de grupo e o grupo vazio numa regra de utilizador",
      "rights": {
        "none": "Nenhum",
        "read": "Leitura",
        "write": "Escrita",
        "full": "Controlo total"
      },
      "viewRights": {
        "none": "Nenhum",
        "read": "Leitura",
        "readWrite": "Leitura/escrita",
        "total": "Controlo total",
        "unknown": "Desconhecido"
      }
    }
  }
} as Messages
