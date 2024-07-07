import React from 'react'


const Header = () => {
  return (
    <header className='lg:sticky lg:top-0 lg:flex lg:max-h-screen lg:w-1/2 lg:flex-col lg:justify-between lg:py-24'>
      <div>
        <h1 className='text-4xl font-bold tracking-tight text-slate-200 sm:text-5xl'>
          <a href='/'>Pravin Kumar</a>
        </h1>
        <h2 className='mt-3 text-lg  font-medium  tracking-tight text-slate-200 sm:text-xl'>Senior Software Engineer</h2>
        <p className='mt-4 max-w-xs leading-normal'>I build web services that are efficient, scalable and maintainable</p>
        <nav className='nav hidden lg:block'>
        <ul className='mt-16 w-max'>
        <li>
          <a href='#about'><span>About</span></a>
        </li>
        <li>
          <a href='#experience'><span>Experience</span></a>
        </li>
        <li>
          <a href='#projects'><span>Projects</span></a>
        </li>
      </ul>
      </nav>
      </div>
    </header>
  )
}

export default Header