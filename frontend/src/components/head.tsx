import { useEffect } from 'react'

import { env } from '@/env'

type HeadProps = {
  title?: string
  description?: string
}

export const Head = ({ title = '', description = '' }: HeadProps = {}) => {
  useEffect(() => {
    document.title = title ? `${title} | ${env.APP_TITLE}` : env.APP_TITLE
  }, [title])

  useEffect(() => {
    if (description) {
      let meta = document.querySelector<HTMLMetaElement>(
        'meta[name="description"]',
      )
      if (!meta) {
        meta = document.createElement('meta')
        meta.name = 'description'
        document.head.appendChild(meta)
      }
      meta.content = description
    }
  }, [description])

  return null
}
