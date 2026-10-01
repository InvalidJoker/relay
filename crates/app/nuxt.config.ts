export default defineNuxtConfig({
  modules: ['@nuxt/ui'],

  devtools: { enabled: true },

  css: ['~/assets/css/main.css'],

  app: {
    head: {
      htmlAttrs: { lang: 'en' },
      link: [{ rel: 'icon', type: 'image/svg+xml', href: '/favicon.svg' }]
    }
  },

  colorMode: {
    preference: 'dark',
    fallback: 'dark'
  },

  ui: {
    theme: {
      colors: ['primary', 'neutral', 'success', 'info', 'warning', 'error']
    }
  },

  fonts: {
    families: [
      { name: 'Geist', provider: 'google' },
      { name: 'Geist Mono', provider: 'google' }
    ]
  },

  routeRules: {
    '/dashboard/**': { ssr: false },
    '/admin/**': { ssr: false }
  },

  nitro: {
    experimental: { asyncContext: true }
  },

  compatibilityDate: '2026-09-01'
})
