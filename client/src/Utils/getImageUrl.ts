
const getImageUrl = (piece :string) => {

    switch(piece) {

        case ' B ':
            return 'bombe.png'

        case 'Mar':
            return '.png'

        case ' G ':
            return '.png'

        case 'Col':
            return '.png'

        case 'Maj':
            return '.png'

        case 'Capt':
            return '.png'

        case ' L ':
            return '.png'

        case 'Ser':
            return '.png'

        case 'Min':
            return '.png'

        case 'Sco':
            return '.png'

        case 'Spy':
            return '.png'

        case ' F ':
            return '.png'

        default:
            return 'default.png'

    }


}

export default getImageUrl
