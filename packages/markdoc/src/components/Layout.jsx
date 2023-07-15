import { useCallback, useEffect, useState } from 'react'
import Link from 'next/link'
import { useRouter } from 'next/router'
import clsx from 'clsx'
import { navigation } from '@/data/navigation'

import { LogomarkHelium, LogoHeliumGovernance } from '@/components/Logo'
import { MobileNavigation } from '@/components/MobileNavigation'
import { Navigation } from '@/components/Navigation'
import { Prose } from '@/components/Prose'
import { Search } from '@/components/Search'


function GitHubIcon(props) {
  return (
    <svg aria-hidden="true" viewBox="0 0 16 16" {...props}>
      <path d="M8 0C3.58 0 0 3.58 0 8C0 11.54 2.29 14.53 5.47 15.59C5.87 15.66 6.02 15.42 6.02 15.21C6.02 15.02 6.01 14.39 6.01 13.72C4 14.09 3.48 13.23 3.32 12.78C3.23 12.55 2.84 11.84 2.5 11.65C2.22 11.5 1.82 11.13 2.49 11.12C3.12 11.11 3.57 11.7 3.72 11.94C4.44 13.15 5.59 12.81 6.05 12.6C6.12 12.08 6.33 11.73 6.56 11.53C4.78 11.33 2.92 10.64 2.92 7.58C2.92 6.71 3.23 5.99 3.74 5.43C3.66 5.23 3.38 4.41 3.82 3.31C3.82 3.31 4.49 3.1 6.02 4.13C6.66 3.95 7.34 3.86 8.02 3.86C8.7 3.86 9.38 3.95 10.02 4.13C11.55 3.09 12.22 3.31 12.22 3.31C12.66 4.41 12.38 5.23 12.3 5.43C12.81 5.99 13.12 6.7 13.12 7.58C13.12 10.65 11.25 11.33 9.47 11.53C9.76 11.78 10.01 12.26 10.01 13.01C10.01 14.08 10 14.94 10 15.21C10 15.42 10.15 15.67 10.55 15.59C13.71 14.53 16 11.53 16 8C16 3.58 12.42 0 8 0Z" />
    </svg>
  )
}

function Header({ navigation }) {
  let [isScrolled, setIsScrolled] = useState(false)
  let router = useRouter()
  let isHomePage = router.pathname === '/'

  useEffect(() => {
    function onScroll() {
      setIsScrolled(window.scrollY > 0)
    }
    onScroll()
    window.addEventListener('scroll', onScroll, { passive: true })
    return () => {
      window.removeEventListener('scroll', onScroll)
    }
  }, [])

  return (
    <header
      className={clsx(
        'sticky top-0 h-14 z-50 flex flex-wrap items-center justify-between bg-white/[var(--bg-opacity-light)] px-4 shadow-md shadow-zinc-900/5 backdrop-blur-sm transition duration-500 dark:shadow-none sm:px-6 lg:px-8',
        isScrolled && isHomePage
          ? 'dark:bg-zinc-800/[var(--bg-opacity-light)] dark:backdrop-blur-md'
          : 'dark:bg-transparent',
        !isHomePage &&
        'dark:bg-zinc-950/[var(--bg-opacity-light)] dark:backdrop-blur-md'
      )}
    >
      <div className="mr-6 flex lg:hidden">
        <MobileNavigation navigation={navigation} />
      </div>
      <div className="relative flex flex-grow basis-0 items-center">
        <Link href="/" aria-label="Home page" className='flex gap-3'>
          <LogomarkHelium className="h-9 w-9" />
          <LogoHeliumGovernance className="hidden lg:block" />
        </Link>
      </div>
      <div className="relative flex basis-0 items-center justify-end gap-4 md:flex-grow">
        <Search />
        <Link href="https://github.com" className="group" aria-label="GitHub">
          <GitHubIcon className="h-6 w-6 fill-zinc-400 group-hover:fill-zinc-500 dark:group-hover:fill-zinc-300" />
        </Link>
      </div>
    </header>
  )
}

function useTableOfContents(tableOfContents) {
  let [currentSection, setCurrentSection] = useState(tableOfContents[0]?.id)

  let getHeadings = useCallback((tableOfContents) => {
    return tableOfContents
      .flatMap((node) => [node.id, ...node.children.map((child) => child.id)])
      .map((id) => {
        let el = document.getElementById(id)
        if (!el) return

        let style = window.getComputedStyle(el)
        let scrollMt = parseFloat(style.scrollMarginTop)

        let top = window.scrollY + el.getBoundingClientRect().top - scrollMt
        return { id, top }
      })
  }, [])

  useEffect(() => {
    if (tableOfContents.length === 0) return
    let headings = getHeadings(tableOfContents)
    function onScroll() {
      let top = window.scrollY
      let current = headings[0].id
      for (let heading of headings) {
        if (top >= heading.top) {
          current = heading.id
        } else {
          break
        }
      }
      setCurrentSection(current)
    }
    window.addEventListener('scroll', onScroll, { passive: true })
    onScroll()
    return () => {
      window.removeEventListener('scroll', onScroll)
    }
  }, [getHeadings, tableOfContents])

  return currentSection
}

export function Layout({ children, title, tableOfContents }) {
  let router = useRouter()
  let isHomePage = router.pathname === '/'
  let allLinks = navigation.flatMap((section) => section.links)
  let linkIndex = allLinks.findIndex((link) => link.href === router.pathname)
  let previousPage = allLinks[linkIndex - 1]
  let nextPage = allLinks[linkIndex + 1]
  let section = navigation.find((section) =>
    section.links.find((link) => link.href === router.pathname)
  )
  let currentSection = useTableOfContents(tableOfContents)

  function isActive(section) {
    if (section.id === currentSection) {
      return true
    }
    if (!section.children) {
      return false
    }
    return section.children.findIndex(isActive) > -1
  }

  return (
    <>
      <Header navigation={navigation} />

      <div className="relative mx-auto flex justify-center sm:px-2 lg:px-0">
        <div className="hidden lg:relative lg:block lg:flex-none">
          <div className="absolute inset-y-0 right-0 w-[50vw] bg-zinc-50 dark:hidden" />
          <div className="absolute bottom-0 right-0 top-16 hidden h-12 w-px bg-gradient-to-t from-zinc-800 dark:block" />
          <div className="absolute bottom-0 right-0 top-28 hidden w-px bg-zinc-800 dark:block" />
          <div className="sticky top-[3rem] -ml-0.5 h-[calc(100vh-3rem)] w-64 overflow-y-auto overflow-x-hidden bg-[#202020] py-10 pl-8 pr-8 xl:w-64 xl:pl-10">
            <Navigation navigation={navigation} />
          </div>
        </div>
        <div className="min-w-0 max-w-2xl flex-auto px-4 py-16 lg:max-w-none lg:px-8 xl:px-16">
          <div className="lg:max-w-5xl">
            <article>
              {(title || section) && (
                <header className="mb-9 space-y-1">
                  {section && (
                    <p className="font-display text-sm font-medium text-teal-500">
                      {section.title}
                    </p>
                  )}
                  {title && (
                    <h1 className="font-display text-3xl tracking-tight text-zinc-900 dark:text-white">
                      {title}
                    </h1>
                  )}
                </header>
              )}
              <Prose>{children}</Prose>
            </article>
          </div>
          <dl className="mt-12 flex pt-6">
            {previousPage && (
              <div>
                <dt>
                  <Link
                    href={previousPage.href}
                    className="inline-flex justify-center gap-0.5 overflow-hidden rounded-full bg-zinc-100 px-3 py-1 text-sm font-medium text-zinc-900 transition hover:bg-zinc-200 dark:bg-zinc-800/40 dark:text-zinc-400 dark:ring-1 dark:ring-inset dark:ring-zinc-800 dark:hover:bg-zinc-800 dark:hover:text-zinc-300"
                  >
                    <span aria-hidden="true">&larr;</span> Previous
                  </Link>
                </dt>
                <dd className="mt-1">
                  <Link
                    tabIndex="-1"
                    aria-hidden="true"
                    href={previousPage.href}
                    className="text-base font-semibold text-zinc-900 transition hover:text-zinc-600 dark:text-white dark:hover:text-zinc-300"
                  >
                    {previousPage.title}
                  </Link>
                </dd>
              </div>
            )}
            {nextPage && (
              <div className="ml-auto text-right">
                <dt>
                  <Link
                    href={nextPage.href}
                    className="inline-flex justify-center gap-0.5 overflow-hidden rounded-full bg-zinc-100 px-3 py-1 text-sm font-medium text-zinc-900 transition hover:bg-zinc-200 dark:bg-zinc-800/40 dark:text-zinc-400 dark:ring-1 dark:ring-inset dark:ring-zinc-800 dark:hover:bg-zinc-800 dark:hover:text-zinc-300"
                  >
                    Next <span aria-hidden="true">&rarr;</span>
                  </Link>
                </dt>
                <dd className="mt-1">
                  <Link
                    tabIndex="-1"
                    aria-hidden="true"
                    href={nextPage.href}
                    className="text-base font-semibold text-zinc-900 transition hover:text-zinc-600 dark:text-white dark:hover:text-zinc-300"
                  >
                    {nextPage.title}
                  </Link>
                </dd>
              </div>
            )}
          </dl>
          <footer className="flex mt-8 flex-col items-center justify-between gap-5 border-t border-zinc-900/5 pt-8 dark:border-white/5 sm:flex-row">
            <p className="text-xs text-zinc-600 dark:text-zinc-400">
              © Copyright {new Date().getFullYear()}. Helium Foundation.
            </p>
            <div className="flex gap-4">
              <a className="group" href="/quickstart#">
                <span className="sr-only">Follow us on Twitter</span>
                <svg
                  viewBox="0 0 20 20"
                  aria-hidden="true"
                  className="h-5 w-5 fill-zinc-700 transition group-hover:fill-zinc-900 dark:group-hover:fill-zinc-500"
                >
                  <path d="M16.712 6.652c.01.146.01.29.01.436 0 4.449-3.267 9.579-9.242 9.579v-.003a8.963 8.963 0 0 1-4.98-1.509 6.379 6.379 0 0 0 4.807-1.396c-1.39-.027-2.608-.966-3.035-2.337.487.097.99.077 1.467-.059-1.514-.316-2.606-1.696-2.606-3.3v-.041c.45.26.956.404 1.475.42C3.18 7.454 2.74 5.486 3.602 3.947c1.65 2.104 4.083 3.382 6.695 3.517a3.446 3.446 0 0 1 .94-3.217 3.172 3.172 0 0 1 4.596.148 6.38 6.38 0 0 0 2.063-.817 3.357 3.357 0 0 1-1.428 1.861 6.283 6.283 0 0 0 1.865-.53 6.735 6.735 0 0 1-1.62 1.744Z"></path>
                </svg>
              </a>
              <a className="group" href="/quickstart#">
                <span className="sr-only">Follow us on GitHub</span>
                <svg
                  viewBox="0 0 20 20"
                  aria-hidden="true"
                  className="h-5 w-5 fill-zinc-700 transition group-hover:fill-zinc-900 dark:group-hover:fill-zinc-500"
                >
                  <path
                    fillRule="evenodd"
                    clipRule="evenodd"
                    d="M10 1.667c-4.605 0-8.334 3.823-8.334 8.544 0 3.78 2.385 6.974 5.698 8.106.417.075.573-.182.573-.406 0-.203-.011-.875-.011-1.592-2.093.397-2.635-.522-2.802-1.002-.094-.246-.5-1.005-.854-1.207-.291-.16-.708-.556-.01-.567.656-.01 1.124.62 1.281.876.75 1.292 1.948.93 2.427.705.073-.555.291-.93.531-1.143-1.854-.213-3.791-.95-3.791-4.218 0-.929.322-1.698.854-2.296-.083-.214-.375-1.09.083-2.265 0 0 .698-.224 2.292.876a7.576 7.576 0 0 1 2.083-.288c.709 0 1.417.096 2.084.288 1.593-1.11 2.291-.875 2.291-.875.459 1.174.167 2.05.084 2.263.53.599.854 1.357.854 2.297 0 3.278-1.948 4.005-3.802 4.219.302.266.563.78.563 1.58 0 1.143-.011 2.061-.011 2.35 0 .224.156.491.573.405a8.365 8.365 0 0 0 4.11-3.116 8.707 8.707 0 0 0 1.567-4.99c0-4.721-3.73-8.545-8.334-8.545Z"
                  ></path>
                </svg>
              </a>
              <a className="group" href="/quickstart#">
                <span className="sr-only">Join our Discord server</span>
                <svg
                  viewBox="0 0 20 20"
                  aria-hidden="true"
                  className="h-5 w-5 fill-zinc-700 transition group-hover:fill-zinc-900 dark:group-hover:fill-zinc-500"
                >
                  <path d="M16.238 4.515a14.842 14.842 0 0 0-3.664-1.136.055.055 0 0 0-.059.027 10.35 10.35 0 0 0-.456.938 13.702 13.702 0 0 0-4.115 0 9.479 9.479 0 0 0-.464-.938.058.058 0 0 0-.058-.027c-1.266.218-2.497.6-3.664 1.136a.052.052 0 0 0-.024.02C1.4 8.023.76 11.424 1.074 14.782a.062.062 0 0 0 .024.042 14.923 14.923 0 0 0 4.494 2.272.058.058 0 0 0 .064-.02c.346-.473.654-.972.92-1.496a.057.057 0 0 0-.032-.08 9.83 9.83 0 0 1-1.404-.669.058.058 0 0 1-.029-.046.058.058 0 0 1 .023-.05c.094-.07.189-.144.279-.218a.056.056 0 0 1 .058-.008c2.946 1.345 6.135 1.345 9.046 0a.056.056 0 0 1 .059.007c.09.074.184.149.28.22a.058.058 0 0 1 .023.049.059.059 0 0 1-.028.046 9.224 9.224 0 0 1-1.405.669.058.058 0 0 0-.033.033.056.056 0 0 0 .002.047c.27.523.58 1.022.92 1.495a.056.056 0 0 0 .062.021 14.878 14.878 0 0 0 4.502-2.272.055.055 0 0 0 .016-.018.056.056 0 0 0 .008-.023c.375-3.883-.63-7.256-2.662-10.246a.046.046 0 0 0-.023-.021Zm-9.223 8.221c-.887 0-1.618-.814-1.618-1.814s.717-1.814 1.618-1.814c.908 0 1.632.821 1.618 1.814 0 1-.717 1.814-1.618 1.814Zm5.981 0c-.887 0-1.618-.814-1.618-1.814s.717-1.814 1.618-1.814c.908 0 1.632.821 1.618 1.814 0 1-.71 1.814-1.618 1.814Z"></path>
                </svg>
              </a>
            </div>
          </footer>
        </div>
        <div className="hidden xl:sticky xl:top-[3rem] xl:-mr-6 xl:block xl:h-[calc(100vh-3rem)] xl:flex-none xl:overflow-y-auto xl:py-16 xl:pr-6 border-l border-zinc-800 pl-6">
          <nav aria-labelledby="on-this-page-title" className="w-56">
            {tableOfContents.length > 0 && (
              <>
                <h2
                  id="on-this-page-title"
                  className="font-display text-sm font-medium text-zinc-900 dark:text-white"
                >
                  On this page
                </h2>
                <ol role="list" className="mt-4 space-y-3 text-sm">
                  {tableOfContents.map((section) => (
                    <li key={section.id}>
                      <h3>
                        <Link
                          href={`#${section.id}`}
                          className={clsx(
                            isActive(section)
                              ? 'text-teal-500'
                              : 'font-normal text-zinc-500 hover:text-zinc-700 dark:text-zinc-400 dark:hover:text-zinc-300'
                          )}
                        >
                          {section.title}
                        </Link>
                      </h3>
                      {section.children.length > 0 && (
                        <ol
                          role="list"
                          className="mt-2 space-y-3 pl-5 text-zinc-500 dark:text-zinc-400"
                        >
                          {section.children.map((subSection) => (
                            <li key={subSection.id}>
                              <Link
                                href={`#${subSection.id}`}
                                className={
                                  isActive(subSection)
                                    ? 'text-teal-500'
                                    : 'hover:text-zinc-600 dark:hover:text-zinc-300'
                                }
                              >
                                {subSection.title}
                              </Link>
                            </li>
                          ))}
                        </ol>
                      )}
                    </li>
                  ))}
                </ol>
              </>
            )}
          </nav>
        </div>
      </div>
    </>
  )
}
