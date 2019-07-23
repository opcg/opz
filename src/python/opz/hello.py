def world():
    print('Hello World!')


def test():
    from rez.resolved_context import ResolvedContext

    context = ResolvedContext(package_requests=['os'])
