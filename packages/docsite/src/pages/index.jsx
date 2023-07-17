import { useState, useEffect } from 'react'
import Head from 'next/head'
import { useRouter } from 'next/router'
import clsx from 'clsx'
import { navigation } from '@/data/navigation'
import { features } from '@/data/home'

import { MobileNavigation } from '@/components/MobileNavigation'
import Showcase from '@/components/Showcase'

import Link from 'next/link'

import { LogomarkHelium, LogoHeliumGovernance } from '@/components/Logo'
import { Search } from '@/components/Search'
import Footer from '@/components/Footer'

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
                'sticky top-0 z-50 flex h-14 flex-wrap items-center justify-between px-4 transition duration-500 sm:px-6 lg:px-8',
                isScrolled
                    ? 'bg-zinc-800/[var(--bg-opacity-light)] backdrop-blur-md'
                    : 'bg-transparent'
            )}
        >
            <div className="mr-6 flex lg:hidden">
                <MobileNavigation navigation={navigation} />
            </div>
            <div className="relative flex flex-grow basis-0 items-center">
                <Link href="/" aria-label="Home page" className="flex gap-3">
                    <LogomarkHelium className="h-9 w-9" />
                    <LogoHeliumGovernance className="hidden lg:block" />
                </Link>
            </div>
            <div className="relative flex basis-0 items-center justify-end gap-4 md:flex-grow">
                <Search />
                <Link href="https://github.com/helium/modular-governance" className="group" aria-label="GitHub">
                    <GitHubIcon className="h-6 w-6 fill-zinc-400 group-hover:fill-zinc-500 dark:group-hover:fill-zinc-300" />
                </Link>
            </div>
        </header>
    )
}

export default function Home() {
    return (
        <>
            <Head>
                <title>Home | Helium Modular Governance</title>
            </Head>
            <div>
                <Header navigation={navigation} />
                {/* ---- HERO --- */}
                <section className="relative flex pb-32 md:min-h-[90vh] flex-col items-center gap-6 overflow-hidden md:overflow-visible px-8 pt-32 lg:pt-36">
                    <svg
                        className="absolute top-0 -z-10 hidden -translate-y-16 dark:block"
                        xmlns="http://www.w3.org/2000/svg"
                        width="1581"
                        height="1184"
                        fill="none"
                    >
                        <path
                            data-aos="fade" data-aos-delay="550"
                            stroke="red" strokeLinejoin="round"
                            className="path origin-center animate-breathe"
                            fill="#1E1E1E"
                            d="M649.235-517.13c87.415-50.493 195.115-50.493 282.53 0l507.965 293.417C1527.15-173.22 1581-79.903 1581 21.083v586.834c0 100.987-53.85 194.302-141.27 244.796L931.765 1146.13c-87.415 50.49-195.115 50.49-282.53 0l-507.97-293.417C53.85 802.219 0 708.904 0 607.917V21.083C0-79.904 53.85-173.22 141.265-223.713l507.97-293.417Z"
                            opacity=".2"
                        />
                        <path
                            data-aos="fade" data-aos-delay="550"
                            className="path origin-center animate-breathe"
                            fill="#212121"
                            d="M676.398-357.403c70.607-40.796 157.598-40.796 228.204 0L1314.9-120.341C1385.5-79.546 1429-4.153 1429 77.438v474.124c0 81.591-43.5 156.984-114.1 197.779L904.602 986.403c-70.606 40.797-157.597 40.797-228.204 0L266.102 749.341C195.496 708.546 152 633.153 152 551.562V77.438c0-81.59 43.496-156.984 114.102-197.779l410.296-237.062Z"
                            opacity=".3"
                        />

                        <path
                            data-aos="fade" data-aos-delay="550"
                            className="path origin-center animate-breathe"
                            fill="#242424"
                            d="M709.994-159.419c49.817-28.775 111.195-28.775 161.012 0L1160.49 7.79C1210.31 36.564 1241 89.742 1241 147.29v334.418c0 57.549-30.69 110.727-80.51 139.501L871.006 788.419c-49.817 28.775-111.195 28.775-161.012 0L420.506 621.21C370.689 592.436 340 539.258 340 481.709V147.291c0-57.549 30.689-110.727 80.506-139.501L709.994-159.42Z"
                            opacity=".4"
                        />
                        <path
                            data-aos="fade" data-aos-delay="550"
                            className="path origin-center animate-breathe"
                            fill="#262626"
                            opacity="1"
                            d="M740.499 23.262a99.013 99.013 0 0 1 99.002 0L1017.5 126.017c30.63 17.683 49.5 50.362 49.5 85.728v205.51c0 35.366-18.87 68.045-49.5 85.728L839.501 605.738a99.012 99.012 0 0 1-99.002 0L562.501 502.983C531.87 485.3 513 452.621 513 417.255v-205.51c0-35.366 18.87-68.045 49.501-85.728L740.499 23.262Z"
                        />
                    </svg>

                    <div className="relative z-10 flex w-full flex-col items-center">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="210"
                            height="230"
                            fill="none"
                            data-aos="fade" data-aos-delay="550"
                            className="relative z-10"
                        >
                            <path
                                fill="url(#a2)"
                                d="M85.054 5.321a39.719 39.719 0 0 1 39.719 0l65.195 37.64a39.72 39.72 0 0 1 19.86 34.398v75.28a39.72 39.72 0 0 1-19.86 34.398l-65.195 37.64a39.715 39.715 0 0 1-39.719 0l-65.194-37.64A39.718 39.718 0 0 1 0 152.639v-75.28a39.72 39.72 0 0 1 19.86-34.398l65.194-37.64Z"
                            />
                            <path
                                className="origin-center animate-slow-bounce"
                                fill="#474DFF"
                                d="M92.005 39.042a25.817 25.817 0 0 1 25.817 0l46.419 26.8A25.817 25.817 0 0 1 177.149 88.2v53.599c0 9.224-4.92 17.747-12.908 22.359l-46.419 26.799a25.816 25.816 0 0 1-25.817 0l-46.418-26.799a25.818 25.818 0 0 1-12.909-22.359v-53.6c0-9.223 4.92-17.746 12.909-22.358l46.418-26.8Z"
                            />
                            <g
                                filter="url(#b)"
                                opacity=".7"
                                className="origin-center animate-slow-bounce"
                            >
                                <path
                                    fill="#000"
                                    fillOpacity=".4"
                                    d="M98.956 109.703 37.39 77.266v82.749l61.565 34.423v-84.735Z"
                                />
                            </g>
                            <g
                                filter="url(#c)"
                                opacity=".7"
                                className="origin-center animate-slow-bounce"
                            >
                                <path
                                    fill="#000"
                                    fillOpacity=".4"
                                    d="m110.872 109.703 61.565-32.437v82.749l-61.565 34.423v-84.735Z"
                                />
                            </g>
                            <defs>
                                <filter
                                    id="b"
                                    width="85.565"
                                    height="141.172"
                                    x="25.391"
                                    y="65.266"
                                    colorInterpolationFilters="sRGB"
                                    filterUnits="userSpaceOnUse"
                                >
                                    <feFlood floodOpacity="0" result="BackgroundImageFix" />
                                    <feBlend
                                        in="SourceGraphic"
                                        in2="BackgroundImageFix"
                                        result="shape"
                                    />
                                    <feGaussianBlur
                                        result="effect1_foregroundBlur_12_70"
                                        stdDeviation="6"
                                    />
                                </filter>
                                <filter
                                    id="c"
                                    width="85.565"
                                    height="141.172"
                                    x="98.872"
                                    y="65.266"
                                    colorInterpolationFilters="sRGB"
                                    filterUnits="userSpaceOnUse"
                                >
                                    <feFlood floodOpacity="0" result="BackgroundImageFix" />
                                    <feBlend
                                        in="SourceGraphic"
                                        in2="BackgroundImageFix"
                                        result="shape"
                                    />
                                    <feGaussianBlur
                                        result="effect1_foregroundBlur_12_70"
                                        stdDeviation="6"
                                    />
                                </filter>
                                <linearGradient
                                    id="a2"
                                    x1="104.914"
                                    x2="104.914"
                                    y1="-6.145"
                                    y2="236.143"
                                    gradientUnits="userSpaceOnUse"
                                >
                                    <stop stopColor="#20DEB0" />
                                    <stop offset="1" stopColor="#C5F7EB" />
                                </linearGradient>
                            </defs>
                        </svg>
                        <h1 data-aos="fade" data-aos-delay="500" className="mt-12 max-w-2xl text-center text-4xl font-bold text-white md:text-6xl">
                            <span className='text-zinc-400'>Unleash Communities with</span> Helium Modular Governance SDK
                        </h1>
                        <div data-aos="fade" data-aos-delay="750" className='mt-10 flex flex-wrap gap-3'>
                            <Link href="/docs/installation"
                                className="rounded-xl bg-teal-500 px-5 py-4 text-lg font-semibold text-teal-950 shadow-sm hover:translate-y-0.5 transition duration-400 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-teal-400">
                                Get started
                            </Link>
                            <Link href="#features"
                                className="rounded-xl bg-zinc-700 px-5 py-4 text-lg font-semibold text-white shadow-sm hover:translate-y-0.5 transition duration-400 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-zinc-400">
                                Learn more
                            </Link>
                        </div>
                    </div>
                </section>

                <section id="features" className="z-10 mt-10 py-24 sm:py-32 gap-6 px-8">
                    <div className="mx-auto max-w-xl sm:text-center mb-12">
                        <h2 data-aos="fade" className="text-base font-semibold leading-7 text-teal-400">Features at glance</h2>
                        <p data-aos="fade" data-aos-delay="150" className="mt-2 text-3xl font-bold tracking-tight text-white sm:text-4xl">
                            No DAO? No problem.
                        </p>
                        <p data-aos="fade" data-aos-delay="250" className="mt-2 text-lg leading-8 text-gray-300">Tweak parameters, upgrade smart contracts and manage your treasury with the flexibility to build your own governance model.</p>
                    </div>
                    <div className="mx-auto grid max-w-4xl grid-cols-1 gap-6 md:grid-cols-3">

                        {features.map((feature, idx) => (
                            <FeatureCard
                                data-aos="fade" data-aos-delay={idx * 50}
                                key={feature.title}
                                icon={feature.icon}
                                title={feature.title}
                                description={feature.description}
                            />
                        ))}
                    </div>
                </section>

                <Showcase />
                <div className='p-8 pt-0'>
                    <Footer noDivider />
                </div>
            </div>
        </>
    )
}


function FeatureCard(props) {
    return (
        <div
            {...props}
            className="flex flex-col gap-2 overflow-hidden rounded-2xl border border-[#333333] bg-[#262626] p-4"
        >
            <div className="bg-neutral/10 flex h-12 w-12 items-center justify-center rounded-full dark:bg-white/10">
                {props.icon}
            </div>
            <h3 className="text-lg font-bold text-black dark:text-white">
                {props.title}
            </h3>
            <p className="pl-1 text-gray-500 dark:text-gray-400">
                {props.description}
            </p>
        </div>
    )
}
