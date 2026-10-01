export default defineAppConfig({
  ui: {
    colors: {
      primary: 'neutral',
      neutral: 'neutral'
    },
    button: {
      slots: {
        base: 'font-medium tracking-tight'
      },
      defaultVariants: {
        size: 'md'
      }
    },
    input: {
      slots: {
        root: 'w-full',
        base: 'bg-transparent'
      },
      defaultVariants: {
        size: 'lg'
      }
    },
    formField: {
      slots: {
        label: 'text-sm font-medium'
      }
    },
    badge: {
      slots: {
        base: 'font-mono uppercase tracking-wider'
      },
      defaultVariants: {
        variant: 'outline',
        color: 'neutral',
        size: 'sm'
      }
    },
    card: {
      slots: {
        root: 'relative',
        header: 'px-5 py-4 sm:px-5',
        body: 'p-5 sm:p-5',
        footer: 'px-5 py-4 sm:px-5 bg-muted'
      }
    },
    modal: {
      slots: {
        overlay: 'bg-black/60 backdrop-blur-[2px]',
        content: 'ring ring-default'
      }
    },
    table: {
      slots: {
        th: 'eyebrow font-normal py-3 bg-muted',
        td: 'text-toned py-3',
        thead: '[&>tr]:after:content-none',
        separator: 'bg-(--ui-border)'
      }
    },
    navigationMenu: {
      slots: {
        link: 'py-2'
      }
    },
    dashboardSidebar: {
      slots: {
        root: 'bg-muted',
        footer: 'border-t border-default py-3'
      }
    },
    dashboardNavbar: {
      slots: {
        title: 'text-sm font-medium'
      }
    },
    tabs: {
      slots: {
        list: 'rounded-none',
        indicator: 'rounded-none'
      }
    },
    toast: {
      slots: {
        root: 'ring ring-default'
      }
    }
  }
})
