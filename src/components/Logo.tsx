import React from 'react'

type LogoProps = {
  text: string
}

const Logo = ({text}: LogoProps) => {
  return (
    <div className='flex items-center'>
        <img src='/portfolio/bat.png' className='w-16 h-16 mr-4'/>
        <span className='text-slate-400 text-xl font-black'>{text}</span>
    </div>
  )
}

export default Logo