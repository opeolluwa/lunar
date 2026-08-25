export const routes = [
  {
    path: "/",
    componentUrl: "/",
  },

  // Auth
  {
    path: "/auth/",
    routes: [
      {
        path: "login",
        componentUrl: "/auth/login.vue",
      },
      {
        path: "signup",
        componentUrl: "/auth/signup.vue",
      },
      {
        path: "confirm-otp",
        componentUrl: "/auth/confirm-otp.vue",
      },
      {
        path: "reset-password",
        componentUrl: "/auth/reset-password.vue",
      },
      {
        path: "accept-invitation",
        componentUrl: "/auth/accept-invitation.vue",
      },
      {
        path: "welcome-back",
        componentUrl: "/auth/welcome-back.vue",
      },
    ],
  },

  // Main
  {
    path: "/bookmarks/",
    routes: [
      {
        path: "",
        componentUrl: "/bookmarks/index.vue",
      },
      {
        path: "create",
        componentUrl: "/bookmarks/create-bookmark.vue",
      },
    ],
  },

  {
    path: "/calendar/",
    componentUrl: "/calendar/index.vue",
  },

  {
    path: "/moodboard/",
    componentUrl: "/moodboard/index.vue",
  },

  {
    path: "/notifications/",
    componentUrl: "/notifications/index.vue",
  },

  {
    path: "/recycle-bin/",
    componentUrl: "/recycle-bin/index.vue",
  },

  {
    path: "/scratch-pad/",
    componentUrl: "/scratch-pad/index.vue",
  },

  // Notes
  {
    path: "/notes/",
    routes: [
      {
        path: "",
        componentUrl: "/notes/index.vue",
      },
      {
        path: "create",
        componentUrl: "/notes/create-notes.vue",
      },
      {
        path: "edit/:id",
        componentUrl: "/notes/edit-notes.vue",
      },
    ],
  },

  // Reminders
  {
    path: "/reminders/",
    routes: [
      {
        path: "",
        componentUrl: "/reminders/index.vue",
      },
      {
        path: "create",
        componentUrl: "/reminders/create-reminder.vue",
      },
      {
        path: "edit/:id",
        componentUrl: "/reminders/edit-reminder.vue",
      },
    ],
  },

  // Snippets
  {
    path: "/snippets/",
    routes: [
      {
        path: "",
        componentUrl: "/snippets/index.vue",
      },
      {
        path: "create",
        componentUrl: "/snippets/create-snippets.vue",
      },
      {
        path: "edit/:id",
        componentUrl: "/snippets/edit-snippet.vue",
      },
      {
        path: "view/:id",
        componentUrl: "/snippets/view-snippet.vue",
      },
    ],
  },

  // Todo
  {
    path: "/todo/",
    routes: [
      {
        path: "",
        componentUrl: "/todo/index.vue",
      },
      {
        path: "create",
        componentUrl: "/todo/create-todo.vue",
      },
      {
        path: "edit/:id",
        componentUrl: "/todo/edit-todo.vue",
      },
    ],
  },

  // Settings
  {
    path: "/settings/",
    routes: [
      {
        path: "",
        componentUrl: "/settings/index.vue",
      },
      {
        path: "about",
        componentUrl: "/settings/about.vue",
      },
      {
        path: "ai",
        componentUrl: "/settings/ai.vue",
      },
      {
        path: "alarm",
        componentUrl: "/settings/alarm.vue",
      },
      {
        path: "appearance",
        componentUrl: "/settings/appearance.vue",
      },
      {
        path: "backup",
        componentUrl: "/settings/backup.vue",
      },
      {
        path: "locale",
        componentUrl: "/settings/locale.vue",
      },
      {
        path: "notifications",
        componentUrl: "/settings/notifications.vue",
      },
      {
        path: "profile",
        componentUrl: "/settings/profile.vue",
      },
      {
        path: "workspaces",
        componentUrl: "/settings/workspaces.vue",
      },
    ],
  },

  {
    path: "/pricing",
    componentUrl: "/pricing.vue",
  },
];
