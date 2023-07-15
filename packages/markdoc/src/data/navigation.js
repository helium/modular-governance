export const navigation = [
    {
      title: 'Introduction',
      icon: <svg width="18" height="21" viewBox="0 0 18 21" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path fillRule="evenodd" clipRule="evenodd" d="M3.78058 7.87629C3.28841 8.25488 3 8.8406 3 9.46154V16.4767C3 17.5813 3.89543 18.4767 5 18.4767H14C15.1046 18.4767 16 17.5813 16 16.4767V9.46154C16 8.8406 15.7116 8.25488 15.2194 7.87629L10.7194 4.41475C10.0005 3.86175 8.99948 3.86175 8.28058 4.41475L3.78058 7.87629Z" fill="#2EE0B5" />
      </svg>,
      links: [
        // { title: 'Getting started', href: '/' },
        { title: 'Installation', href: '/docs/installation' },
        { title: 'Intro', href: '/docs/intro' },
      ],
    },
    {
      title: 'API',
      icon: <svg xmlns="http://www.w3.org/2000/svg" width="18" height="21" fill="none"><path fill="#2EE0B5" stroke="#2EE0B5" d="M12.5 5v9a1.5 1.5 0 0 1-1.5 1.5H4A1.5 1.5 0 0 1 2.5 14V5A1.5 1.5 0 0 1 4 3.5h7A1.5 1.5 0 0 1 12.5 5Z"/><path fill="#2EE0B5" stroke="#2EE0B5" d="M15.5 8v9a1.5 1.5 0 0 1-1.5 1.5H7A1.5 1.5 0 0 1 5.5 17V8A1.5 1.5 0 0 1 7 6.5h7A1.5 1.5 0 0 1 15.5 8Z" opacity=".4"/></svg>,
      links: [
        // DOCS NAVIGATION START
        { title: 'Voter', href: '/docs/api/nft-voter-sdk' },
        { title: 'Organization', href: '/docs/api/organization-sdk' },
        { title: 'Proposals', href: '/docs/api/proposal-sdk' },
        { title: 'State controller', href: '/docs/api/state-controller-sdk' },
        { title: 'Token voter', href: '/docs/api/token-voter-sdk' },
      ],
    }
  ]