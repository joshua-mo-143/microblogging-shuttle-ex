"use client"
import Image from 'next/image'
import Link from 'next/link'

export default function Navbar() {
  return (
    <nav className="">
      <ul className="text-xl font-bold flex flex-row justify-start gap-4">
        <li className="transition-all hover:underline hover:underline-offset-4">
          <Link href="/posts">Explore</Link>
        </li>
        <li className="transition-all hover:underline hover:underline-offset-4">
          <Link href="/login">Register/Login</Link>
        </li>
      </ul>
    </nav>
  )
}
