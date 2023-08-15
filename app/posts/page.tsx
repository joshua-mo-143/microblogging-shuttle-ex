import Image from 'next/image'
import Navbar from '@/components/Navbar'
import TitleHeader from '@/components/TitleHeader'

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col w-1/2 p-24 gap-10">
      <TitleHeader/>
      <Navbar/>
      <section className="text-2xl flex flex-col gap-4">
        <h1 className="font-bold">Posts</h1>
        <div>
          <p className="text-lg"> No posts :(</p>
      </div>
      </section>
    </main>
  )
}
