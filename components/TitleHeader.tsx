import Image from 'next/image'
import Link from 'next/link'

export default function TitleHeader() {
  return (
      <section className="font-bold">
    <Link href="/">
      <h1 className="text-4xl">Minidiary</h1>
    </Link>
      <p>The easiest way to microblog.</p>
    </section>
  )
}
