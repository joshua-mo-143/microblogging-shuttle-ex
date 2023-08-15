import Image from 'next/image'
import Navbar from '@/components/Navbar'
import TitleHeader from '@/components/TitleHeader'

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col w-1/2 p-24 gap-10">
      <TitleHeader/>
      <Navbar/>
      <section className="text-2xl">
        <p>It&apos;s <span className="font-bold">easier than ever</span> to get started on your microblog.</p>
      <ul className="list-inside list-disc text-xl">
          <li>
            A blog that <span className="font-bold">just works.</span>
        </li>
          <li>
            Responsiveness across <span className="font-bold">all</span> devices.
        </li>
          <li>
            Sign up in seconds, stay for a while.
        </li>
          <li>
            Built to last using Rust
        </li> 
      </ul>
        </section>
    </main>
  )
}
