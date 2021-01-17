
const getImageUrl = (piece :string) => {

    const baseUrl = 'assets/images/'

    switch(piece) {

        case ' B ':
            return baseUrl + 'bombe.png'

        case 'Mar':
            return baseUrl + 'marshal.png'

        case ' G ':
            return baseUrl + 'general.png'

        case 'Col':
            return baseUrl + 'colonel.png'

        case 'Maj':
            return baseUrl + 'major.png'

        case 'Capt':
            return baseUrl + 'captain.png'

        case ' L ':
            return baseUrl + 'lieutenant.png'

        case 'Ser':
            return baseUrl + 'sergeant.png'

        case 'Min':
            return baseUrl + 'miner.png'

        case 'Sco':
            return baseUrl + 'scout.png'

        case 'Spy':
            return baseUrl + 'spy.png'

        case ' F ':
            return baseUrl + 'flag.png'

        default:
            return baseUrl + ''

    }


}

export default getImageUrl
