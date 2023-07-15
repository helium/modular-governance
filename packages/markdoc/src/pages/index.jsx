import { useState, useEffect } from 'react'
import { useRouter } from 'next/router'
import clsx from 'clsx'

import { MobileNavigation } from '@/components/MobileNavigation'

import Link from 'next/link'

import { LogomarkHelium, LogoHeliumGovernance } from '@/components/Logo'
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

export default function Home({ Component, pageProps }) {
    return (
        <div>
            <Header/>
            <h1>Home</h1>
        </div>
    )
}