"use client"
import Image from 'next/image'
import Link from 'next/link'
import Navbar from '@/components/Navbar'
import {useSearchParams, useRouter} from 'next/navigation'
import React from 'react'
import {accountStore} from '@/zustandStore'

export default function Login() {

  const params = useSearchParams()
  const code = params.get("code")

  const setDisplayName = accountStore();

  console.log(params.get("code"))

  React.useEffect(() => {
    const fetch = async () => {
    let res = await fetch("localhost:8000/api/auth/google_callback",
    {
        mode: 'cors',
        withCredentials: 'true',
        headers: {
          "content-type": "application/json",
        },
        body: JSON.stringify({
          code: code
        })
      });

    if (res.ok) {
      
      router.push("/")
    } else {
      router.push("/login")
    }
    };
    fetch()
  }, [])
  
    return (
    <main className="flex min-h-screen flex-col w-1/2 p-24 gap-10">
      <section className="font-bold">
    
      <h1 className="text-4xl">Minidiary</h1>
      <p>The easiest way to microblog.</p>
    </section>
      <Navbar/>
      <section className="text-2xl flex flex-col gap-10">
           <p>Logging you in...</p> 
        </section>
    </main>
  )
}
