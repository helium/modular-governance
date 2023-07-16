import Link from 'next/link'
import { useRouter } from 'next/router'
import clsx from 'clsx'

export function Navigation({ navigation, className }) {
  let router = useRouter()

  return (
    <nav className={clsx('text-base lg:text-sm', className)}>
      <ul role="list" className="space-y-6">
        {navigation.map((section) => (
          <li key={section.title}>
            <h2 className="font-display font-medium text-zinc-900 dark:text-white flex gap-2">
              {section.icon && section.icon}
              {section.title}
            </h2>
            <ul
            id="trail-nav"
              role="list"
               className="mt-2 relative space-y-2 lg:mt-2 ml-2 lg:space-y-1 lg:border-zinc-200"
            >
              {section.links.map((link) => (
                <li key={link.href} className="relative">
                  <Link
                    href={link.href}
                    className={clsx(
                      'trail-nav-link',
                      'block w-full pl-3.5 before:ring-[#202020] before:ring-2',
                      link.href === router.pathname
                        ? 'font-semibold text-teal-300'
                        : 'text-zinc-500 hover:text-zinc-600 dark:text-zinc-400 dark:hover:text-zinc-300'
                    )}
                  >
                    {link.title}
                  </Link>
                </li>
              ))}
            </ul>
          </li>
        ))}
      </ul>
    </nav>
  )
}
