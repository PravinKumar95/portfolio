import React from 'react'
import Link from './Link'

const Navigation = () => {
  return (
    <nav className='text-blue-200'>
      <ul className='flex items-center justify-between'>
      <li className='p-4'><Link text='Work' url=''/></li>
        <li className='p-4'><Link text='About' url=''/></li>
        
      </ul>
    </nav>
  )
}

export default Navigation